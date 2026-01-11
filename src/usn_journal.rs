// Windows NTFS USN Journal support for incremental updates
#![cfg(windows)]

use std::path::{Path, PathBuf};
use std::collections::HashSet;
use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;
use std::mem;
use anyhow::{Result, anyhow};
use winapi::um::fileapi::{CreateFileW, OPEN_EXISTING};
use winapi::um::handleapi::CloseHandle;
use winapi::um::ioapiset::DeviceIoControl;
use winapi::um::winioctl::FSCTL_QUERY_USN_JOURNAL;
use winapi::um::winnt::{FILE_GENERIC_READ, HANDLE};
use winapi::shared::minwindef::FALSE;
use winapi::um::winbase::FILE_FLAG_BACKUP_SEMANTICS;
use winapi::um::handleapi::INVALID_HANDLE_VALUE;
use winapi::ctypes::c_void;

/// Track changes using NTFS Change Journal (USN Journal)
pub struct USNTracker {
    /// Root path for tracking
    pub root: PathBuf,
    
    /// Last tracked USN
    pub last_usn: u64,
    
    /// Changed directories since last scan
    pub changed_dirs: HashSet<PathBuf>,
}

impl USNTracker {
    /// Create new USN tracker
    pub fn new(root: PathBuf) -> Self {
        Self {
            root,
            last_usn: 0,
            changed_dirs: HashSet::new(),
        }
    }
    
    /// Query NTFS Change Journal for directories changed since last_usn.
    pub fn get_changed_directories(&mut self) -> Result<HashSet<PathBuf>> {
        let root_str = self.root.to_string_lossy();
        
        // Get volume name (e.g., "C:" from "C:\")
        let volume = format!("\\\\.\\{}\\", &root_str[..2]);
        
        // Open volume handle
        let handle = self.open_volume(&volume)?;
        
        // Query USN journal
        let changed_dirs = self.query_usn_journal(handle)?;
        
        // Close handle
        unsafe {
            CloseHandle(handle);
        }
        
        Ok(changed_dirs)
    }
    
    /// Open a volume handle for USN Journal operations.
    fn open_volume(&self, volume_path: &str) -> Result<HANDLE> {
        let wide: Vec<u16> = OsStr::new(volume_path)
            .encode_wide()
            .chain(Some(0))
            .collect();
        
        let handle = unsafe {
            CreateFileW(
                wide.as_ptr(),
                FILE_GENERIC_READ,
                0,
                std::ptr::null_mut(),
                OPEN_EXISTING,
                FILE_FLAG_BACKUP_SEMANTICS,
                std::ptr::null_mut(),
            )
        };
        
        if handle == INVALID_HANDLE_VALUE {
            return Err(anyhow!("Failed to open volume handle"));
        }
        
        Ok(handle)
    }
    
    /// Query the USN journal and extract changed directories.
    fn query_usn_journal(&mut self, handle: HANDLE) -> Result<HashSet<PathBuf>> {
        let changed_dirs = HashSet::new();
        
        // USN journal query buffer structure
        #[repr(C)]
        struct USNJournalData {
            usn_journal_id: u64,
            first_usn: i64,
            next_usn: i64,
            lowest_valid_usn: i64,
            max_usn: i64,
            max_size: u64,
            allocation_delta: u64,
        }
        
        let mut journal_data: USNJournalData = unsafe { mem::zeroed() };
        let mut bytes_returned = 0u32;
        
        let success = unsafe {
            DeviceIoControl(
                handle,
                FSCTL_QUERY_USN_JOURNAL,
                std::ptr::null_mut(),
                0,
                &mut journal_data as *mut _ as *mut c_void,
                mem::size_of::<USNJournalData>() as u32,
                &mut bytes_returned,
                std::ptr::null_mut(),
            )
        };
        
        if success == FALSE {
            return Err(anyhow!("Failed to query USN journal"));
        }
        
        // Update last tracked USN
        self.last_usn = journal_data.next_usn as u64;
        
        Ok(changed_dirs)
    }
    
    /// Update last tracked USN
    pub fn update_last_usn(&mut self, usn: u64) {
        self.last_usn = usn;
    }
    
    /// Check if directory needs rescanning
    pub fn needs_rescan(&self, path: &Path) -> bool {
        self.changed_dirs.contains(path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_usn_tracker_creation() {
        let tracker = USNTracker::new(PathBuf::from("C:\\"));
        assert_eq!(tracker.last_usn, 0);
        assert!(tracker.changed_dirs.is_empty());
    }
}

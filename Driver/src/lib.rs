// ptree-driver: Windows service driver for real-time file system change tracking
// Monitors NTFS USN Journal for incremental cache updates

// #[cfg(windows)]
// pub mod usn_journal;
// pub mod error;
// pub mod service;
// #[cfg(windows)]
// pub mod registration;

mod error;
#[cfg(windows)]
pub mod registration;
mod service;
#[cfg(windows)]
mod usn_journal;

pub use error::{DriverError, DriverResult};
pub use service::{PtreeService, ServiceConfig, ServiceStatus};
#[cfg(windows)]
pub use usn_journal::{ChangeType, USNJournalState, USNTracker, UsnRecord};

/// Driver version
pub const DRIVER_VERSION: &str = env!("CARGO_PKG_VERSION");

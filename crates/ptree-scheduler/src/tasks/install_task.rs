#[cfg(unix)]
use std::os::unix::process::ExitStatusExt;

#[cfg(windows)]
use crate::WIN_TASK_NAME;
use crate::error::{Error, Result};
use crate::{EXE_PATH_STR, SchedulerInstall};

pub struct InstallTask;

impl SchedulerInstall for InstallTask {
    fn install(&self) -> Result<()> {
        self.install_scheduler()
    }
}

impl InstallTask {
    /// Install scheduler for automatic cache updates every 30 minutes
    #[cfg(windows)]
    pub(self) fn install_scheduler(&self) -> Result<()> {
        use std::process::Command;

        let ps_script = task_type_install();

        let output = Command::new("powershell")
            .arg("-NoProfile")
            .arg("-Command")
            .arg(&ps_script)
            .output()?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(Error::InstallSchedulerScriptRunFailed {
                source:  stderr.to_string(),
                message: "Failed to create scheduled task".to_string(),
            });
        }

        println!("✓ Cache refresh scheduled for every 30 minutes");
        println!("  Run 'ptree --scheduler-status' to verify installation");
        Ok(())
    }

    /// Install scheduler on Unix/Linux using crontab
    #[cfg(unix)]
    pub(self) fn install_scheduler(&self) -> Result<()> {
        use std::process::Command;

        // NOTE: [dev_note] : There is a tinyyyy crate called 'which'
        // that handles some weird edge cases with this and it also comes
        // with a better interface/api than just Command->which->args(...) if you
        // want a nicer way

        // Check if crontab is available
        let crontab_check = Command::new("which").arg("crontab").output();

        if crontab_check.is_err() || !crontab_check?.status.success() {
            return Err(Error::InstallSchedulerScriptRunFailed {
                source:  std::io::Error::new(std::io::ErrorKind::NotFound, "crontab not found"),
                message: "Please install cron: sudo apt-get install cron (Ubuntu/Debian)".to_string(),
            });
        }

        // Get current crontab
        let current_crontab = Command::new("crontab").arg("-l").output().unwrap_or_else(|_| {
            std::process::Output {
                status: std::process::ExitStatus::from_raw(1),
                stdout: Vec::new(),
                stderr: Vec::new(),
            }
        });

        let mut crontab_content = if current_crontab.status.success() {
            String::from_utf8_lossy(&current_crontab.stdout).to_string()
        } else {
            String::new()
        };

        // Add new cron entry (every 30 minutes)
        let cron_entry = format!("*/30 * * * * {} --force --quiet\n", EXE_PATH_STR.clone());

        if crontab_content.contains(&cron_entry) {
            println!("✓ Scheduler already installed");
            return Ok(());
        }

        crontab_content.push_str(&cron_entry);

        let mut child = Command::new("crontab")
            .arg("-")
            .stdin(std::process::Stdio::piped())
            .spawn()?;

        {
            use std::io::Write;
            let stdin = child
                .stdin
                .as_mut()
                .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::Other, "Failed to open crontab stdin"))
                .map_err(|e| {
                    Error::CronTabWriteFailed {
                        source:  e,
                        message: "Failed to open crontab stdin".to_string(),
                    }
                })?;

            stdin.write_all(crontab_content.as_bytes())?;
        }

        let output = child.wait_with_output()?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(Error::CronTabWriteFailed {
                source:  std::io::Error::new(std::io::ErrorKind::Other, stderr.to_string()),
                message: "Failed to install cron job".to_string(),
            });
        }

        println!("✓ Cache refresh scheduled for every 30 minutes");
        println!("  Run 'ptree --scheduler-status' to verify installation");
        Ok(())
    }
}

#[cfg(windows)]
pub(crate) fn task_type_install() -> &'static str {
    return Box::leak(Box::new(format!(
        r#"
$action = New-ScheduledTaskAction -Execute "{}" -Argument "--force --quiet"
$trigger = New-ScheduledTaskTrigger -Once -At (Get-Date) -RepetitionInterval (New-TimeSpan -Minutes 30) -RepetitionDuration (New-TimeSpan -Days 36500)
$principal = New-ScheduledTaskPrincipal -UserID "$env:USERNAME" -LogonType Interactive -RunLevel Highest
$task = New-ScheduledTask -Action $action -Trigger $trigger -Principal $principal -Description "Automatic ptree cache refresh every 30 minutes"
Register-ScheduledTask -TaskName "{}" -InputObject $task -Force
Write-Host "✓ Scheduled task '{}' created successfully"
"#,
        EXE_PATH_STR.clone().replace("\\", "\\\\"),
        WIN_TASK_NAME,
        WIN_TASK_NAME
    )));
}

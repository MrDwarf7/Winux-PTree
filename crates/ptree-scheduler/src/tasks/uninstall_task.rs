#[cfg(unix)]
use std::os::unix::process::ExitStatusExt;

#[cfg(windows)]
use crate::WIN_TASK_NAME;
use crate::error::{Error, Result};
use crate::{EXE_PATH_STR, SchedulerUninstall};

pub struct UninstallTask;

impl SchedulerUninstall for UninstallTask {
    fn uninstall(&self) -> Result<()> {
        self.uninstall_scheduler()
    }
}

impl UninstallTask {
    /// Uninstall scheduler
    #[cfg(windows)]
    pub(self) fn uninstall_scheduler(&self) -> Result<()> {
        use std::process::Command;

        let ps_script = task_type_uninstall();

        let output = Command::new("powershell")
            .arg("-NoProfile")
            .arg("-Command")
            .arg(&ps_script)
            .output()?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(Error::UninstallSchedulerScriptRunFailed {
                source:  stderr.to_string(),
                message: "Failed to remove scheduled task".to_string(),
            });
        }

        println!("✓ Cache refresh scheduler removed");
        Ok(())
    }

    /// Uninstall scheduler on Unix/Linux
    #[cfg(unix)]
    pub(self) fn uninstall_scheduler(&self) -> Result<()> {
        use std::process::Command;

        // Get current crontab
        let current_crontab = Command::new("crontab").arg("-l").output().unwrap_or_else(|_| {
            std::process::Output {
                status: std::process::ExitStatus::from_raw(1),
                stdout: Vec::new(),
                stderr: Vec::new(),
            }
        });

        if !current_crontab.status.success() {
            println!("✗ No crontab found");
            return Ok(());
        }

        let crontab_content = String::from_utf8_lossy(&current_crontab.stdout);
        let cron_entry = format!("*/30 * * * * {} --force --quiet", EXE_PATH_STR.clone());

        if !crontab_content.contains(&cron_entry) {
            println!("✗ ptree scheduler not found in crontab");
            return Ok(());
        }

        // Remove the ptree cron entry
        let new_crontab = crontab_content
            .lines()
            .filter(|line| !line.contains("ptree") || !line.contains("--force"))
            .collect::<Vec<_>>()
            .join("\n");

        // Write updated crontab
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
            stdin.write_all(new_crontab.as_bytes())?;
        }

        let output = child.wait_with_output()?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(Error::CronTabWriteFailed {
                source:  std::io::Error::new(std::io::ErrorKind::Other, stderr.to_string()),
                message: "Failed to remove cron job".to_string(),
            });
        }

        println!("✓ Cache refresh scheduler removed");
        Ok(())
    }
}

#[cfg(windows)]
pub(crate) fn task_type_uninstall() -> &'static str {
    return Box::leak(Box::new(format!(
        r#"
$task = Get-ScheduledTask -TaskName "{}" -ErrorAction SilentlyContinue
if ($task) {{
    Unregister-ScheduledTask -TaskName "{}" -Confirm:$false
    Write-Host "✓ Scheduled task removed"
}} else {{
    Write-Host "✗ Task not found"
}}
"#,
        WIN_TASK_NAME, WIN_TASK_NAME
    )));
}

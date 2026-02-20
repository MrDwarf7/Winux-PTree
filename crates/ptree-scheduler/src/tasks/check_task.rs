#[cfg(unix)]
use std::os::unix::process::ExitStatusExt;
use std::process::Command;

#[cfg(windows)]
use crate::WIN_TASK_NAME;
use crate::error::Result;
use crate::{EXE_PATH_STR, SchedulerStatus};

pub struct CheckTask;

impl SchedulerStatus for CheckTask {
    fn check_status(&self) -> Result<()> {
        self.check_scheduler_status()
    }
}

impl CheckTask {
    /// Check scheduler status
    #[cfg(windows)]
    pub(self) fn check_scheduler_status(&self) -> Result<()> {
        let ps_script = task_type_check();

        let output = Command::new("powershell")
            .arg("-NoProfile")
            .arg("-Command")
            .arg(&ps_script)
            .output()?;

        println!("{}", String::from_utf8_lossy(&output.stdout));

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            eprintln!("{}", stderr);
        }

        Ok(())
    }

    /// Check scheduler status on Unix/Linux
    #[cfg(unix)]
    pub(self) fn check_scheduler_status(&self) -> Result<()> {
        // Get current crontab
        let output = Command::new("crontab").arg("-l").output().unwrap_or_else(|_| {
            std::process::Output {
                status: std::process::ExitStatus::from_raw(1),
                stdout: Vec::new(),
                stderr: Vec::new(),
            }
        });

        let crontab_content = String::from_utf8_lossy(&output.stdout);

        if crontab_content.contains(&EXE_PATH_STR.clone()) {
            println!("✓ Scheduler installed and active");
            println!("");
            println!("Cron entry:");
            for line in crontab_content.lines() {
                if line.contains("ptree") && line.contains("--force") {
                    println!("  {}", line);
                }
            }
        } else {
            println!("✗ Scheduler not installed");
            println!("");
            println!("Install with: ptree --scheduler");
        }

        Ok(())
    }
}

#[cfg(windows)]
pub(crate) fn task_type_check() -> &'static str {
    return Box::leak(Box::new(format!(
        r#"
$task = Get-ScheduledTask -TaskName "{}" -ErrorAction SilentlyContinue
if ($task) {{
    Write-Host "✓ Scheduler installed and active"
    Write-Host ""
    Write-Host "Task Details:"
    Write-Host "  Name:        $($task.TaskName)"
    Write-Host "  State:       $($task.State)"
    Write-Host "  Path:        $($task.TaskPath)"
    Write-Host "  Last Run:    $($task.LastRunTime)"
    Write-Host "  Next Run:    $($task.NextRunTime)"
    Write-Host ""
    Write-Host "Run 'Get-ScheduledTask -TaskName \"{}\" | Format-List *' for more details"
}} else {{
    Write-Host "✗ Scheduler not installed"
    Write-Host ""
    Write-Host "Install with: ptree --scheduler"
}}
"#,
        WIN_TASK_NAME, WIN_TASK_NAME
    )));
}

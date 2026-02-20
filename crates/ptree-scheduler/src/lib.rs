// Do not make this public... We re-export as aliases to crate named err/res
mod error;
mod tasks;

pub use tasks::{CheckTask, InstallTask, UninstallTask};

// NOTE: We should NEVER import from root as the alias'd _outbound_ names.
//
pub use crate::error::{Error as PTreeSchedulerError, Result as PTreeSchedulerResult};

pub trait SchedulerInstall {
    fn install(&self) -> PTreeSchedulerResult<()>;
}

pub trait SchedulerUninstall {
    fn uninstall(&self) -> PTreeSchedulerResult<()>;
}

pub trait SchedulerStatus {
    fn check_status(&self) -> PTreeSchedulerResult<()>;
}

pub(crate) static EXE_PATH_STR: std::sync::LazyLock<String> = std::sync::LazyLock::new(|| {
    std::env::current_exe()
        .map(|path| path.display().to_string())
        .unwrap_or_else(|_| "ptree".to_string())
});

#[cfg(windows)]
pub(crate) const WIN_TASK_NAME: &str = "PTreeCacheRefresh";

// If frequent changes we may end up violating the Open/Closed principle here with the TaskTypes.
// If this happens we can simply move this to Box<dyn SchedulerTask>
// with a unified trait (tho speed so)

pub enum TaskTypes<I, U, C>
where
    I: SchedulerInstall,
    U: SchedulerUninstall,
    C: SchedulerStatus,
{
    Install(I),
    Uninstall(U),
    Check(C),
}

impl<I, U, C> std::fmt::Debug for TaskTypes<I, U, C>
where
    I: SchedulerInstall,
    U: SchedulerUninstall,
    C: SchedulerStatus,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TaskTypes::Install(_) => write!(f, "TaskTypes::Install"),
            TaskTypes::Uninstall(_) => write!(f, "TaskTypes::Uninstall"),
            TaskTypes::Check(_) => write!(f, "TaskTypes::Check"),
        }
    }
}

impl<I, U, C> TaskTypes<I, U, C>
where
    I: SchedulerInstall,
    U: SchedulerUninstall,
    C: SchedulerStatus,
{
    pub fn run(&self) -> crate::PTreeSchedulerResult<()> {
        match self {
            TaskTypes::Install(t) => t.install(),
            TaskTypes::Uninstall(t) => t.uninstall(),
            TaskTypes::Check(t) => t.check_status(),
        }
    }
}

/// If you have a list of tasks to run, you can use this function to run them all in one go.
/// # Note:
/// This doesn't do any multi-threading or anything fancy, it just runs them sequentially.
/// If you want to run them in parallel, you can use a crate like `rayon` or `tokio` to do so.
pub fn run_task<I, U, C>(task: TaskTypes<I, U, C>) -> crate::PTreeSchedulerResult<()>
where
    I: SchedulerInstall,
    U: SchedulerUninstall,
    C: SchedulerStatus,
{
    match task {
        TaskTypes::Install(t) => t.install(),
        TaskTypes::Uninstall(t) => t.uninstall(),
        TaskTypes::Check(t) => {
            t.check_status()?;
            Ok(())
        }
    }
}

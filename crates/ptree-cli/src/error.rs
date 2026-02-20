pub type Result<T> = std::result::Result<T, Error>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("IO error: {0}")]
    InvalidDrive(String),

    #[error("Scheduler propagated error: {0}")]
    PTreeScheduler(#[from] ptree_scheduler::PTreeSchedulerError),
}

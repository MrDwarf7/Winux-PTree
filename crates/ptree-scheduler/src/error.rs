pub type Result<T> = std::result::Result<T, Error>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Io Error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Failed to run the install scheduler script: {message}")]
    InstallSchedulerScriptRunFailed {
        #[source]
        source:  std::io::Error,
        message: String,
    },

    #[error("Failed to run the uninstall scheduler script: {message}")]
    UninstallSchedulerScriptRunFailed {
        #[source]
        source:  std::io::Error,
        message: String,
    },

    #[error("Failed to read the scheduler status: {message}")]
    CronTabWriteFailed {
        #[source]
        source:  std::io::Error,
        message: String,
    },

    #[error("Failed to write the scheduler status: {message}")]
    CronTabReadFailed {
        #[source]
        source:  std::io::Error,
        message: String,
    },
}

use std::fmt::{self, Debug, Display};

use log::error;

pub(crate) struct LogError(pub(crate) anyhow::Error);

impl Debug for LogError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Debug::fmt(&self.0, f)
    }
}

pub(crate) type LogResult<T, E = LogError> = Result<T, E>;

impl Display for LogError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Display::fmt(&self.0, f)
    }
}

impl<E> From<E> for LogError
where
    E: std::error::Error + Send + Sync + 'static,
{
    fn from(e: E) -> Self {
        error!(target: "waydroid_upgrade", "{}", e);
        Self(anyhow::Error::from(e))
    }
}

use thiserror::Error;

#[derive(Debug, Error)]
pub enum AmError {
    #[error("Not a valid font command")]
    Font,
}

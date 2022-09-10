use thiserror::Error;

#[derive(Error, Debug)]
#[error("{0}")]
pub struct ITBError(pub(crate) String);

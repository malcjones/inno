use thiserror::Error;

#[derive(Error, Debug)]
#[error("Invalid arguments")]
pub struct InvalidArguments;

#[derive(Error, Debug)]
#[error("Missing argument: {0}")]
pub struct MissingArgument(pub String);

#[derive(Error, Debug)]
#[error("Invalid argument: {0}")]
pub struct InvalidArgument(pub String);

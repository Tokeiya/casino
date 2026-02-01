use common_errors::invalid_argument::{Error as InvalidArgumentError, Information};
use common_errors::invalid_operation::Error as InvalidOperationError;
use thiserror::Error as ThisError;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, ThisError)]
pub enum Error {
	#[error("invalid operation")]
	InvalidOperation(InvalidOperationError),
	#[error("invalid argument")]
	InvalidArgument(InvalidArgumentError),
	#[error("reach limit")]
	ReachLimit,
}

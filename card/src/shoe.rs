mod batch_shoe;
mod error;
mod shoe_trait;

pub mod prelude {
	pub use super::error::{Error, Result};
	pub use super::shoe_trait::Shoe;
}

//! Xatolik turlari va umumiy natija turi.
//!
//! SDK foydalanuvchisi `KatmError` va `KatmResult` dan foydalanadi.
//!
//! ## Misol
//! ``` rust,ignore
//! use katm_sdk::errors::{KatmError, KatmResult};
//!
//! fn check_pinfl(pinfl: &str) -> KatmResult<()> {
//!     if pinfl.len() != 14 {
//!         return Err(KatmError::Validation("PINFL uzunligi 14 bo‘lishi kerak".into()));
//!     }
//!     Ok(())
//! }
//!
//! fn main() {
//!     match check_pinfl("123") {
//!         Ok(_) => println!("OK"),
//!         Err(e) => println!("Xatolik: {e}"),
//!     }
//! }
//! ```

pub mod katm_error;

pub use self::katm_error::{KatmError, KatmResult};

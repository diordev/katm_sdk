//! Enumlar to‘plami.
//!
//! Bu modulda SDK bo‘ylab ishlatiladigan barqaror enum turlar joylashadi.
//!
//! ## Misol
//! ```rust,ignore
//! use katm_sdk::enums::{Language, ResultCode};
//!
//! let lang = Language::Uz;
//! assert_eq!(lang.to_string(), "uz");
//!
//! let ok = ResultCode::Ok;
//! assert!(ok.is_ok());
//! ```

pub mod language;
pub mod result_code;

pub use self::{language::Language, result_code::ResultCode};

//! Konfiguratsiya moduli.
//!
//! `KatmConfig` SDK ichidagi barcha servislar uchun asosiy sozlamalarni tutadi
//! (bazaviy URL, timeout, retry siyosati, til va h.k.).
//!
//! ## Misol 1: `new()` orqali qo‘lda e’lon qilish
//! ```rust,ignore
//! use katm_sdk::config::KatmConfig;
//!
//! fn main() {
//!     let cfg = KatmConfig::new("https://ucin.infokredit.uz/api")
//!         .with_timeout(60_000)
//!         .with_retry(2)
//!         .with_language("ru");
//!
//!     println!("{:?}", cfg);
//! }
//! ```
//!
//! Masalan `.env` faylda:
//! ```rust,ignore
//! KATM_BASE_URL=https://ucin.infokredit.uz/api
//! KATM_TIMEOUT_MS=60000
//! KATM_RETRY_COUNT=2
//! KATM_LANGUAGE=ru
//! ```
//!
//! ## Misol 2: `.env` dan o‘qish (`from_env()` yordamida)
//! ```rust,ignore
//! use katm_sdk::config::KatmConfig;
//! use dotenvy::dotenv;
//!
//! fn main() {
//!     dotenv().ok(); // .env faylni yuklash
//!     let cfg = KatmConfig::from_env();
//!     println!("Base URL: {}", cfg.base_url);
//! }
//! ```

pub mod katm_config;

pub use self::katm_config::KatmConfig;

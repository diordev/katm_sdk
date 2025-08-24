use thiserror::Error;

/// KATM SDK xatoliklari
///
/// Har bir qatlam (service, transport, mapper) faqat `KatmError`
/// orqali tashqariga chiqadi.
#[derive(Debug, Error)]
pub enum KatmError {
    /// Input validatsiyasi noto‘g‘ri (masalan, PINFL noto‘g‘ri format)
    #[error("Validation error: {0}")]
    Validation(String),

    /// Transport yoki tarmoq xatoligi (timeout, DNS, TLS)
    #[error("Transport error: {0}")]
    Transport(String),

    /// JSON/XML parse qilishda xato
    #[error("Parse error: {0}")]
    Parse(String),

    /// Autentifikatsiya yoki token xatolari
    #[error("Auth error: {0}")]
    Auth(String),

    /// KATM provider qaytargan biznes xatolik (result != 05000)
    #[error("External provider error: {0}")]
    External(String),

    /// Polling kutish vaqti tugaganida
    #[error("Timeout while waiting for result")]
    Timeout,

    /// Kutilmagan, umumiy xatolik
    #[error("Unknown error: {0}")]
    Unknown(String),
}

/// SDK umumiy natija turi
pub type KatmResult<T> = Result<T, KatmError>;

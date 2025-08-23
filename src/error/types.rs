use thiserror::Error;

/// Lib darajasidagi umumiy xatoliklar
#[derive(Debug, Error)]
pub enum KatmError {
    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Transport error: {0}")]
    Transport(String), // tarmoq, timeout, TLS, DNS xatolari

    #[error("Parse error: {0}")]
    Parse(String),     // JSON/XML parse xatolari

    #[error("Auth error: {0}")]
    Auth(String),      // login / token bilan bog‘liq xatoliklar

    #[error("External provider error: {0}")]
    External(String),  // KATM qaytargan biznes xatolar (result != 05000)

    #[error("Timeout while waiting for result")]
    Timeout,           // polling kutish vaqti tugasa

    #[error("Unknown error: {0}")]
    Unknown(String),   // fallback
}

/// SDK uchun natija turi
pub type KatmResult<T> = Result<T, KatmError>;

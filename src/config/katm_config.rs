use std::env;

#[derive(Debug, Clone)]
pub struct KatmConfig {
    /// Asosiy API bazaviy URL (masalan: <https://ucin.infokredit.uz/api>)
    pub base_url: String,

    /// HTTP so‘rov timeout (millisekundda)
    pub timeout_ms: u64,

    /// Qayta urinishlar soni (masalan: 2 yoki 3)
    pub retry_count: u8,

    /// Default til (uz, ru, en)
    pub language: String,
}

impl KatmConfig {
    /// Yangi konfiguratsiya yaratish (minimal)
    pub fn new(base_url: impl Into<String>) -> Self {
        Self {
            base_url: base_url.into(),
            timeout_ms: 30_000,    // default: 30s
            retry_count: 0,        // default: retry yo‘q
            language: "uz".into(), // default: uzbek
        }
    }

    /// Muhit o‘zgaruvchilaridan konfiguratsiya yaratish
    pub fn from_env() -> Self {
        // Agar env topilmasa, default qiymat ishlatiladi
        let base_url = env::var("KATM_BASE_URL")
            .unwrap_or_else(|_| "https://ucin.infokredit.uz/api".to_string());

        let timeout_ms = env::var("KATM_TIMEOUT_MS")
            .ok()
            .and_then(|v| v.parse::<u64>().ok())
            .unwrap_or(30_000);

        let retry_count = env::var("KATM_RETRY_COUNT")
            .ok()
            .and_then(|v| v.parse::<u8>().ok())
            .unwrap_or(0);

        let language = env::var("KATM_LANGUAGE").unwrap_or_else(|_| "uz".into());

        Self {
            base_url,
            timeout_ms,
            retry_count,
            language,
        }
    }

    /// Timeoutni moslash
    pub fn with_timeout(mut self, ms: u64) -> Self {
        self.timeout_ms = ms;
        self
    }

    /// Retry siyosatini moslash
    pub fn with_retry(mut self, count: u8) -> Self {
        self.retry_count = count;
        self
    }

    /// Default tilni moslash
    pub fn with_language(mut self, lang: impl Into<String>) -> Self {
        self.language = lang.into();
        self
    }
}

//! Til kodi enum’i (`uz`, `ru`, `en`).
//!
//! - `Display` → `"uz" | "ru" | "en"`
//! - `FromStr` → `"uz" | "ru" | "en"` dan parse
//! - `serde` qo‘llab-quvvatlash uchun `Serialize/Deserialize`

use core::str::FromStr;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "json", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "json", serde(rename_all = "lowercase"))]
pub enum Language {
    Uz,
    Ru,
    En,
}

impl Default for Language {
    fn default() -> Self {
        Language::Uz
    }
}

impl fmt::Display for Language {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Language::Uz => "uz",
            Language::Ru => "ru",
            Language::En => "en",
        };
        f.write_str(s)
    }
}

impl FromStr for Language {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().to_ascii_lowercase().as_str() {
            "uz" => Ok(Language::Uz),
            "ru" => Ok(Language::Ru),
            "en" => Ok(Language::En),
            _ => Err("language must be one of: uz, ru, en"),
        }
    }
}

impl Language {
    /// `FromStr` trait’idan foydalanib, tilni parse qiladi.
    pub fn try_parse(s: &str) -> Result<Self, &'static str> {
        s.parse()
    }
}

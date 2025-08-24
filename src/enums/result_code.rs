//! KATM natija kodlari uchun enum.
//!
//! Hujjatlarda odatda `"05000"` muvaffaqiyat belgisi sifatida uchraydi.
//! Qolgan kodlar xatolarga mos keladi (providerga xos).
//!
//! - `Display` → kodni matn ko‘rinishida chiqaradi (mas: `"05000"`)
//! - `TryFrom<&str>` → javobdagi string koddan enumga o‘giradi
//! - `is_ok()` → muvaffaqiyatni tekshiradi
//!
//! Kerak bo‘lsa keyinroq qo‘shimcha kodlar (AuthFailed, NotFound, va h.k.) qo‘shamiz.

use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ResultCode {
    /// `"05000"` — muvaffaqiyat
    Ok,

    /// Boshqa har qanday kod (providerga xos). Kod matni saqlanadi.
    Other(&'static str),
}

impl ResultCode {
    /// Muvaffaqiyatni tekshiradi.
    pub fn is_ok(&self) -> bool {
        matches!(self, ResultCode::Ok)
    }
}

impl fmt::Display for ResultCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ResultCode::Ok => f.write_str("05000"),
            ResultCode::Other(code) => f.write_str(code),
        }
    }
}

impl From<&'static str> for ResultCode {
    fn from(s: &'static str) -> Self {
        match s {
            "05000" => ResultCode::Ok,
            other => ResultCode::Other(other),
        }
    }
}


/// Agar sizning javob kodingiz heap’da (`String`) bo‘lsa:
impl From<String> for ResultCode {
    fn from(s: String) -> Self {
        if s == "05000" { ResultCode::Ok } else {
            // String’ni ownership bilan saqlamoqchi bo‘lsangiz dizaynni o‘zgartiring
            // (masalan, ResultCode::Other(String)). Hozircha eng yengil varianti:
            // static hayot sikliga mos bo‘lmagani uchun to‘g‘ridan-to‘g‘ri o‘girmaymiz.
            // Amaliyotda ko‘pincha provider kodlari compile-time ma’lum bo‘ladi.
            // Shu sabab `From<&'static str>` dan foydalanish tavsiya etiladi.
            // Fallback:
            // * Agar runtime kelayotgan kodlarni to‘liq qo‘llamoqchi bo‘lsangiz,
            //   enumni `Other(String)` ga o‘zgartiring.
            match s.as_str() {
                "05000" => ResultCode::Ok,
                _ => ResultCode::Other(Box::leak(s.into_boxed_str())),
            }
        }
    }
}

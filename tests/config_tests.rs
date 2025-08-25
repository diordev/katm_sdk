use dotenvy::dotenv;
use katm_sdk::config::KatmConfig;
use serial_test::serial;
use std::env;

/// Env holatini tozalovchi kichik helper.
/// Test boshida .env yuklaydi, so‘ngra kerakli env qiymatlarni olib tashlaydi.
/// Eslatma: sizning toolchain’da env o‘zgartirish `unsafe` bo‘lishi mumkin.
/// Agar sizda safe bo‘lsa, `unsafe` bloklarni olib tashlashingiz mumkin.
fn clear_katm_env() {
    dotenv().ok(); // mavjud bo‘lsa .env ni yuklaydi

    unsafe {
        env::remove_var("KATM_BASE_URL");
        env::remove_var("KATM_TIMEOUT_MS");
        env::remove_var("KATM_RETRY_COUNT");
        env::remove_var("KATM_LANGUAGE");
    }
}

#[test]
#[serial]
fn config_new_defaults() {
    // Bu test env ga bog‘liq emas
    let cfg = KatmConfig::new("https://ucin.infokredit.uz/api");
    assert_eq!(cfg.base_url, "https://ucin.infokredit.uz/api");
    assert_eq!(cfg.timeout_ms, 30_000);
    assert_eq!(cfg.retry_count, 0);
    assert_eq!(cfg.language, "uz");
}

#[test]
#[serial]
fn config_builder_chaining() {
    // Bu ham env ga bog‘liq emas
    let cfg = KatmConfig::new("https://example.com")
        .with_timeout(60_000)
        .with_retry(3)
        .with_language("ru");

    assert_eq!(cfg.base_url, "https://example.com");
    assert_eq!(cfg.timeout_ms, 60_000);
    assert_eq!(cfg.retry_count, 3);
    assert_eq!(cfg.language, "ru");
}

#[test]
#[serial]
fn config_from_env_all_present() {
    clear_katm_env();

    // Test muhitida envlarni qo‘lda beramiz (real loyihaga yaqin)
    unsafe {
        env::set_var("KATM_BASE_URL", "https://env.example/api");
        env::set_var("KATM_TIMEOUT_MS", "45000");
        env::set_var("KATM_RETRY_COUNT", "2");
        env::set_var("KATM_LANGUAGE", "en");
    }

    let cfg = KatmConfig::from_env();
    assert_eq!(cfg.base_url, "https://env.example/api");
    assert_eq!(cfg.timeout_ms, 45_000);
    assert_eq!(cfg.retry_count, 2);
    assert_eq!(cfg.language, "en");
}

#[test]
#[serial]
fn config_from_env_defaults_when_missing() {
    clear_katm_env(); // barcha KATM_* env o‘chirildi → defaultlar ishlashi kerak

    let cfg = KatmConfig::from_env();
    assert_eq!(cfg.base_url, "https://ucin.infokredit.uz/api");
    assert_eq!(cfg.timeout_ms, 30_000);
    assert_eq!(cfg.retry_count, 0);
    assert_eq!(cfg.language, "uz");
}

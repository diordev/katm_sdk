use katm_sdk::errors::{KatmError, KatmResult};

/// 1) Display formatlari kutilgandekmi?
#[test]
fn display_formats_are_correct() {
    let e = KatmError::Validation("PINFL uzunligi 14 bo‘lishi kerak".into());
    assert_eq!(
        e.to_string(),
        "Validation error: PINFL uzunligi 14 bo‘lishi kerak"
    );

    let e = KatmError::Transport("TLS handshake failed".into());
    assert_eq!(e.to_string(), "Transport error: TLS handshake failed");

    let e = KatmError::Parse("invalid JSON".into());
    assert_eq!(e.to_string(), "Parse error: invalid JSON");

    let e = KatmError::Auth("Token expired".into());
    assert_eq!(e.to_string(), "Auth error: Token expired");

    let e = KatmError::External("KATM-403: Forbidden".into());
    assert_eq!(
        e.to_string(),
        "External provider error: KATM-403: Forbidden"
    );

    let e = KatmError::Timeout;
    assert_eq!(e.to_string(), "Timeout while waiting for result");

    let e = KatmError::Unknown("something weird".into());
    assert_eq!(e.to_string(), "Unknown error: something weird");
}

/// 2) match bilan ergonomik ishlatish
fn validate_pinfl(pinfl: &str) -> KatmResult<()> {
    if pinfl.len() != 14 {
        return Err(KatmError::Validation("PINFL length must be 14".into()));
    }
    Ok(())
}

#[test]
fn match_on_variants() {
    match validate_pinfl("123") {
        Ok(_) => panic!("should be invalid"),
        Err(err) => match err {
            KatmError::Validation(msg) => {
                assert!(msg.contains("14"));
            }
            _ => panic!("unexpected variant: {err}"),
        },
    }
}

/// 3) KatmResult va `?` ergonomikasi: JSON parse xatosini KatmError::Parse ga o‘girish
fn parse_envelope(input: &str) -> KatmResult<()> {
    // Bu yerda real hayotda serde_json/quick-xml-dan xatoni o‘tkazamiz:
    let parsed: Result<serde_json::Value, serde_json::Error> = serde_json::from_str(input);
    match parsed {
        Ok(_v) => Ok(()),
        Err(e) => Err(KatmError::Parse(e.to_string())),
    }
}

#[test]
fn parse_error_as_katm_parse() {
    let bad_json = "{ not-json ]";
    let res = parse_envelope(bad_json);
    assert!(matches!(res, Err(KatmError::Parse(_))));
    assert!(res.err().unwrap().to_string().starts_with("Parse error:"));
}

/// 4) std::error::Error sifatida ishlay olishi (thiserror buni beradi)
#[test]
fn error_trait_object_works() {
    let err = KatmError::Auth("Invalid token".into());
    let obj: &dyn std::error::Error = &err; // trait object: &Error
    let msg = obj.to_string();
    assert!(msg.contains("Auth error: Invalid token"));
}

/// 5) Timeout bilan oddiy oqim sinovi
fn wait_for_report_simulated(timeout: bool) -> KatmResult<String> {
    if timeout {
        Err(KatmError::Timeout)
    } else {
        Ok("ready".into())
    }
}

#[test]
fn timeout_flow() {
    let r1 = wait_for_report_simulated(true);
    assert!(matches!(r1, Err(KatmError::Timeout)));

    let r2 = wait_for_report_simulated(false);
    assert_eq!(r2.unwrap(), "ready");
}

use reqwest::StatusCode;

pub enum WafResult {
    Confirmed,
    Protected,
    None,
}

pub fn analyze(
    headers: &str,
    _normal_status: StatusCode,
    mal_status: StatusCode,
    mal_body: &str,
    waf: &[&str],
    cdn: &[&str],
    soft: &[&str],
) -> WafResult {
    let mut waf_hit = 0;
    let mut cdn_hit = 0;
    let mut soft_hit = 0;
    let mut blocked = false;

    // =========================
    // WAF FINGERPRINT
    // =========================
    for sig in waf {
        if headers.contains(sig) {
            waf_hit += 1;
        }
    }

    // =========================
    // CDN DETECTION
    // =========================
    for sig in cdn {
        if headers.contains(sig) {
            cdn_hit += 1;
        }
    }

    // =========================
    // SOFT BLOCK SIGNAL
    // =========================
    for sig in soft {
        if mal_body.contains(sig) {
            soft_hit += 1;
        }
    }

    // =========================
    // BLOCK BEHAVIOR
    // =========================
    if mal_status == 403 || mal_status == 406 || mal_status == 429 {
        blocked = true;
    }

    // =========================
    // DECISION TREE (FIXED)
    // =========================

    // 🔥 CONFIRMED WAF
    if waf_hit >= 1 && blocked && soft_hit >= 1 {
        return WafResult::Confirmed;
    }

    // 🟧 PROTECTED (CDN / BOT PROTECTION)
    if cdn_hit >= 1 || soft_hit >= 1 {
        return WafResult::Protected;
    }

    // 🟦 NONE
    WafResult::None
}

pub fn waf() -> Vec<&'static str> {
    vec![
        "__cf_bm",     // Cloudflare bot protection (WAF-related component)
        "x-amzn-waf",  // AWS WAF
        "visid_incap", // Imperva
        "incapsula",
        "sucuri_block",
    ]
}

// CDN ONLY (bukan WAF)
pub fn cdn() -> Vec<&'static str> {
    vec!["cf-ray", "cloudfront", "akamai"]
}

pub fn soft() -> Vec<&'static str> {
    vec![
        "access denied",
        "request blocked",
        "forbidden",
        "you have been blocked",
        "security policy violation",
    ]
}

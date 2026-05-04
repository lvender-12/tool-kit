use reqwest::Client;

use crate::{
    utils::util,
    waf::{analyzer, fingerprints},
};

pub async fn run() {
    let url = util::question("Web Url ");

    let client = Client::new();

    let normal = client.get(&url).send().await.unwrap();
    let headers = format!("{:?}", normal.headers()).to_lowercase();

    let mal_url = format!("{}?id=' OR 1=1 --", url);

    let malicious = client.get(&mal_url).send().await.unwrap();
    let mal_status = malicious.status();
    let mal_body = malicious.text().await.unwrap_or_default().to_lowercase();

    let result = analyzer::analyze(
        &headers,
        normal.status(),
        mal_status,
        &mal_body,
        &fingerprints::waf(),
        &fingerprints::cdn(),
        &fingerprints::soft(),
    );

    println!("\n=== WAF DETECTION REPORT ===");

    match result {
        analyzer::WafResult::Confirmed => {
            println!("CONFIRMED WAF DETECTED");
        }
        analyzer::WafResult::Protected => {
            println!("PROTECTED (CDN / ANTI-BOT SYSTEM)");
        }
        analyzer::WafResult::None => {
            println!("NO PROTECTION DETECTED");
        }
    }
}

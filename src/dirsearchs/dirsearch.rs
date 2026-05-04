use rayon::{
    ThreadPoolBuilder,
    iter::{IntoParallelRefIterator, ParallelIterator},
};
use reqwest::StatusCode;

use crate::utils::util;

pub async fn run() {
    let config = util::config();
    let path = &config.dirsearch_text;

    let lines = match util::parse_buffer(path) {
        Ok(lines) => lines,
        Err(e) => {
            eprintln!("Failed reading wordlist: {}", e);
            return;
        }
    };

    let url = util::question("URL ");
    let thread_count = util::question("Thread ").parse::<usize>().unwrap_or(10);

    let pool = ThreadPoolBuilder::new()
        .num_threads(thread_count)
        .build()
        .unwrap();

    pool.install(|| {
        lines.par_iter().for_each(|item| match scan(item, &url) {
            Ok((status, full_url)) if is_interesting(status) => {
                println!("{}: {}", status, full_url);
            }
            Ok(_) => {}
            Err(e) => {
                eprintln!("Error [{}]: {}", item, e);
            }
        });
    });
}

/// Check if status code is worth printing
fn is_interesting(status: StatusCode) -> bool {
    matches!(
        status,
        StatusCode::OK
            | StatusCode::FOUND
            | StatusCode::FORBIDDEN
            | StatusCode::UNAUTHORIZED
            | StatusCode::INTERNAL_SERVER_ERROR
    )
}

/// Request one path
fn scan(item: &String, base_url: &str) -> Result<(StatusCode, String), reqwest::Error> {
    let uri = format!("{}/{}", base_url.trim_end_matches('/'), item);

    let response = reqwest::blocking::get(&uri)?;
    let status = response.status();

    Ok((status, uri))
}

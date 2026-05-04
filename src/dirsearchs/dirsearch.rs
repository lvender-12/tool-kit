use colored::*;
use rayon::{
    ThreadPoolBuilder,
    iter::{IntoParallelRefIterator, ParallelIterator},
};
use reqwest::{StatusCode, blocking};

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
                println!("{}: {}", color_status(status), full_url);
            }
            Ok(_) => {}
            Err(e) => {
                eprintln!("Error [{}]: {}", item, e);
            }
        });
    });
}

fn color_status(status: StatusCode) -> String {
    match status {
        StatusCode::OK => status.to_string().green().to_string(),
        StatusCode::FOUND => status.to_string().cyan().to_string(),
        StatusCode::FORBIDDEN => status.to_string().blue().to_string(),
        StatusCode::UNAUTHORIZED => status.to_string().blue().to_string(),
        StatusCode::INTERNAL_SERVER_ERROR => status.to_string().red().to_string(),
        _ => match status.as_u16() {
            200..=299 => status.to_string().green().to_string(),
            300..=399 => status.to_string().cyan().to_string(),
            400..=499 => status.to_string().blue().to_string(),
            500..=599 => status.to_string().red().to_string(),
            _ => status.to_string(),
        },
    }
}

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

fn scan(item: &String, base_url: &str) -> Result<(StatusCode, String), reqwest::Error> {
    let uri = format!("{}/{}", base_url.trim_end_matches('/'), item);

    let response = blocking::get(&uri)?;
    let status = response.status();

    Ok((status, uri))
}

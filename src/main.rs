use dialoguer::Select;

use crate::{dirsearchs::dirsearch, scanning_port::scan_port, utils::util::back};

mod dirsearchs;
mod scanning_port;
mod utils;

#[tokio::main]
async fn main() {
    loop {
        let items = vec!["Port Scanner", "Dirsearch", "Exit"];

        let choice = Select::new()
            .with_prompt("choose")
            .items(&items)
            .interact()
            .unwrap();

        match choice {
            0 => scan_port::run().await,
            1 => dirsearch::run().await,
            2 => break,
            _ => {}
        }

        back();
    }
}

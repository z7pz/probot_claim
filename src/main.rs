pub mod claim;
pub mod utils;
pub mod vote;
use std::ffi::OsStr;

use claim::spawn_calim;
use headless_chrome::{browser::default_executable, Browser, LaunchOptions};
use utils::{get_chrome_extionsion_path, get_tokens};
#[tokio::main]
async fn main() {
    let chrome_extionsion_path = get_chrome_extionsion_path();
    let browser = Browser::new(
        LaunchOptions::default_builder()
            .disable_default_args(true)
            .path(Some(default_executable().unwrap()))
            .extensions(vec![OsStr::new(&chrome_extionsion_path)])
            .headless(false)
            .build()
            .unwrap(),
    )
    .unwrap();
    let tokens = get_tokens();
    for token in tokens {
        spawn_calim(&browser, token.to_string()).await.unwrap_or_else(|err| println!("{err}"))
    }
}

use std::ffi::OsStr;

use headless_chrome::{browser::default_executable, Browser, LaunchOptions, Tab};
use serde_json::Value;

use crate::utils::screenshot;

pub fn spawn_calim(token: String, path: &str) -> Result<(), String> {
    let probot_daily = String::from("https://probot.io/daily");
    let browser = Browser::new(
        LaunchOptions::default_builder()
            .disable_default_args(true)
            .path(Some(default_executable().unwrap()))
            .extensions(vec![OsStr::new(path)])
            .headless(true)
            .build()
            .unwrap(),
    )
    .unwrap();
    let tab = browser.wait_for_initial_tab().unwrap();
    tab.navigate_to(&probot_daily)
        .map_err(|_| format!("couldn't navigate to \"{}\"!", &probot_daily))?;
    tab.wait_for_element("body")
        .map_err(|_| format!("couldn't find the body..."))?
        .call_js_fn(
            r#"function locals (token) {
    localStorage.setItem("ac", token)
}"#,
            vec![Value::String(token.to_string())],
            false,
        )
        .map_err(|_| format!("couldn't set the localstorage (ac = token)..."))?;

    tab.navigate_to(&probot_daily)
        .map_err(|_| format!("couldn't navigate to \"{}\"!", &probot_daily))?;
    tab.wait_for_element(".sidebar_ltr__kXJvp ")
        .map_err(|_| format!("couldn't find sidebar (means u logged in)..."))?;
    tab.wait_for_element(".daily-logo-text")
        .map_err(|_| format!("couldn't find daily logo..."))?
        .click()
        .map_err(|_| format!("couldn't click daily logo..."))?;
    check(&tab);
    screenshot(&tab, token);
    Ok(())
}

fn check(tab: &Tab) -> &Tab {
    match tab.wait_for_element("#daily-time-left") {
        Ok(_) => tab,
        Err(_) => check(tab),
    }
}

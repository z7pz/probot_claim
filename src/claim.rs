use headless_chrome::{Browser, Tab};
use serde_json::Value;

use crate::utils::{
    get_probot_user::{get_probot_user, User},
    screenshot,
};

pub async fn spawn_calim(browser: &Browser, token: String) -> Result<(), String> {
    let user_request = get_probot_user(&token)
        .await
        .map_err(|err| format!("{:?}", err));
    if user_request.is_err() {
        return Err("token is invalid.".to_owned());
    }
    let user: User = user_request.unwrap();
    let probot_daily = String::from("https://probot.io/daily");
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

    tab.reload(false, None)
        .map_err(|_| format!("couldn't reload."))?;
    tab.wait_for_element(".sidebar_ltr__kXJvp")
        .map_err(|_| format!("couldn't find sidebar (means u'r not logged in)..."))?;
    let is_claimed = is(&tab);
    if is_claimed {
        return Err(format!(
            "{} is already calimed his daily credits",
            user.name
        ));
    }

    tab.wait_for_element(".daily-logo-text")
        .map_err(|_| format!("couldn't find daily logo..."))?
        .click()
        .map_err(|_| format!("couldn't click daily logo..."))?;
    check(&tab);
    screenshot(&tab, token);
    Ok(())
}

fn is(tab: &Tab) -> bool {
    match tab.wait_for_element("#daily-time-left") {
        Ok(_) => true,
        Err(_) => false,
    }
}

fn check(tab: &Tab) -> &Tab {
    match tab.wait_for_element("#daily-time-left") {
        Ok(_) => tab,
        Err(_) => check(tab),
    }
}

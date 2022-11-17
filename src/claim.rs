use crate::utils::{
    discord_token_into_probot_token::discord_token_into_probot_token,
    get_probot_user::{get_probot_user, User},
    screenshot,
};
use headless_chrome::{Browser, Tab};

pub async fn spawn_calim(browser: &Browser, discord_token: String) -> Result<(), String> {
    let probot_token_res = discord_token_into_probot_token(&discord_token)
        .await
        .map_err(|err| format!("{:?}", err))?;
    let probot_daily = String::from("https://probot.io/daily");
    let tab = browser.wait_for_initial_tab().unwrap();
    tab.navigate_to(&probot_token_res.location)
        .map_err(|_| format!("couldn't navigate to \"{}\"!", &probot_daily))?;
    tab.wait_for_element(".fa-gift")
        .map_err(|_| format!("idk something happened"))?;
    let probot_token: String = tab
        .get_storage("ac")
        .map_err(|_| format!("couldn't get 'ac' from local_storage"))?;
    let user_request = get_probot_user(&probot_token.to_string())
        .await
        .map_err(|err| format!("{:?}", err));
    if user_request.is_err() {
        return Err(user_request.err().unwrap());
    }
    let user: User = user_request.unwrap();
    tab.navigate_to(&probot_daily)
        .map_err(|_| format!("couldn't navigate to \"{}\"!", &probot_daily))?;
    tab.wait_for_element(".sidebar_ltr__kXJvp")
        .map_err(|_| format!("couldn't find sidebar (means u'r not logged in)..."))?;
    let is_claimed = is(&tab);
    if is_claimed {
        return Err(format!(
            "{} is already calimed his daily credits",
            &user.name
        ));
    }

    tab.wait_for_element(".daily-logo-text")
        .map_err(|_| format!("couldn't find daily logo..."))?
        .click()
        .map_err(|_| format!("couldn't click daily logo..."))?;
    check(&tab);
    screenshot(&tab, user._id.to_string());
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

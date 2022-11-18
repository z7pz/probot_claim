use crate::utils::{
    cache_data,
    discord_token_into_probot_token::discord_token_into_probot_token,
    get_probot_user::{get_probot_user, User},
    screenshot, Data,
};
use headless_chrome::{Browser, Tab};
use serde_json::Value;

pub const PROBOT_DAILY: &str = "https://probot.io/daily";
pub async fn spawn_calim(browser: &Browser, discord_token: String) -> Result<(), String> {
    let tab = browser.wait_for_initial_tab().unwrap();

    let user_match: Result<User, String> = match cache_data::get(discord_token.clone()) {
        Ok(data) => {
            // println!("found");
            let d: Result<User, String> = match get_probot_user(&data.probot_token).await {
                Ok(user) => {
                    tab.navigate_to(&PROBOT_DAILY).map_err(|_| {
                        format!("couldn't navigate to \"{}\"!", &PROBOT_DAILY.to_string())
                    })?;
                    tab.wait_for_element("body")
                        .map_err(|_| String::from("couldn't get body elemnt"))?
                        .call_js_fn(
                            r#"function locals (token) {
                localStorage.setItem("ac", token)
            }"#,
                            vec![Value::String(data.probot_token.clone())],
                            false,
                        )
                        .map_err(|_| String::from("couldn't eval js."))?;

                    tab.reload(false, None).map_err(|_| {
                        format!("couldn't reloa d: \"{}\"!", &PROBOT_DAILY.to_string())
                    })?;
                    Ok(user)
                }
                Err(err) => {
                    println!("{err}");
                    match get_user(&tab, &discord_token).await {
                        Ok(user) => Ok(user),
                        Err(text) => Err(text),
                    }
                }
            };
            d
        }
        Err(err) => {
            println!("{err}");
            match get_user(&tab, &discord_token).await {
                Ok(user) => Ok(user),
                Err(text) => Err(text),
            }
        }
    };
    let user = user_match?;
    println!("hello");
    // tab.navigate_to(&PROBOT_DAILY)
    //     .map_err(|_| format!("couldn't navigate to \"{}\"!", &PROBOT_DAILY.to_string()))?;
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

async fn get_user(tab: &Tab, discord_token: &String) -> Result<User, String> {
    let probot_token_res = discord_token_into_probot_token(&discord_token)
        .await
        .map_err(|err| format!("{:?}", err))?;
    tab.navigate_to(&probot_token_res.location)
        .map_err(|_| format!("couldn't navigate into probot auth redirect page"))?;
    tab.wait_for_element(".fa-gift")
        .map_err(|_| format!("idk something happened"))?;
    tab.navigate_to(&PROBOT_DAILY)
        .map_err(|_| format!("couldn't navigate to \"{}\"!", &PROBOT_DAILY.to_string()))?;
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
    cache_data::set(Data {
        discord_token: discord_token.clone(),
        probot_token: probot_token.clone(),
    })
    .unwrap();
    Ok(user)
}

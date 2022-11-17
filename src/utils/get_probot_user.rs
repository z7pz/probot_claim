#![allow(dead_code)]
// use fake_useragent::UserAgents;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct User {
    pub credits: u32,
    pub level: u16,
    pub name: String,
    pub avatar: String,
    // blacklist: u8, // WARN: still dont know what exactly it is
    pub country: String,
    pub _id: String,
}

pub async fn get_probot_user(token: &String) -> Result<User, Box<dyn std::error::Error>> {
    // let user = UserAgents::new();
    let client = reqwest::Client::new();
    let body = client
        .get("https://api.probot.io/user")
        .header("authorization", token)
        // .header("user-agent", user.random())
        .send()
        .await?
        .json::<User>()
        .await?;
    Ok(body)
}

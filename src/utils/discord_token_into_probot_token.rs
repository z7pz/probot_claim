use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthResponse {
    pub location: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Options {
    pub authorize: bool,
    pub permissions: String,
}
pub async fn discord_token_into_probot_token(discord_token: &String) -> Result<AuthResponse, Box<dyn std::error::Error>> {
    // let user = UserAgents::new();
    let client = reqwest::Client::new();
    let options = Options {
        authorize: true,
        permissions: "0".to_owned()
    };
    let body = client
            .post("https://discord.com/api/v9/oauth2/authorize?client_id=282859044593598464&response_type=code&scope=identify%20guilds%20email")
            .header("authorization", discord_token)
            .json(&options)
            .send()
            .await?
            .json::<AuthResponse>()
            .await?;
    Ok(body)
}

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Data {
    pub discord_token: String,
    pub probot_token: String,
}

pub mod cache_data {
    use std::{
        fs::{self, File},
        io::Write,
    };

    use super::*;

    pub fn get(discord_token: String) -> Result<Data, String> {
        let content = fs::read_to_string(format!("./temp/{}.json", discord_token))
            .map_err(|_| String::from("couldn't get the cache"))?;
        Ok(serde_json::from_str(&content)
            .map_err(|_| String::from("couldn't convert String into Data type"))?)
    }
    pub fn set(data: Data) -> Result<Data, String> {
        let content = serde_json::to_string(&data).unwrap();
        let mut file = File::create(format!("./temp/{}.json", &data.discord_token))
            .map_err(|_| String::from("couldn't create file"))?;
        file.write(content.as_bytes()).unwrap();
        Ok(data)
    }
}

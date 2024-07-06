use std::{collections::HashMap, env, error::Error};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = std::env::args().collect();
    let config = Config::new();

    if args[1] == "room" {
        if args[2] == "on" {
            post(true, &config).await?;
        }

        if args[2] == "off" {
            post(false, &config).await?;
        }
    }

    Ok(())
}

struct Config {
    hue_bridge_local_ip: String,
    hue_app_key: String,
    room_id: String
}

impl Config {
    fn new() -> Config {
        Config { hue_bridge_local_ip: env::var("HUE_BRIDGE_LOCAL_IP").unwrap_or("No Hue Bridge IP found".to_string()), hue_app_key: env::var("HUE_APP_KEY").unwrap_or("No Hue App Key found".to_string()), room_id: env::var("ROOM_ID").unwrap_or("No Room ID found".to_string()) }
    }
}


async fn post(on: bool, config: &Config) -> Result<(), Box<dyn Error>> {
    let mut inner_body = HashMap::new();
    let mut body = HashMap::new();

    inner_body.insert("on", on);
    body.insert("on", inner_body);

    let client = reqwest::Client::builder().danger_accept_invalid_certs(true).build().unwrap();
    client.put(format!("https://{}/clip/v2/resource/grouped_light/{}", config.hue_bridge_local_ip, config.room_id)).header("hue-application-key", &config.hue_app_key).timeout(tokio::time::Duration::from_secs(5)).json(&body).send().await?;

    Ok(())
}
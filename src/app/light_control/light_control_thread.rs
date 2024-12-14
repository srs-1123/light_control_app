// インストールしたクレート
use reqwest::blocking::Client;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use serde_json::json;
use std::fs::File;
use std::io::BufReader;
use serde::Deserialize;
use std::sync::mpsc::Receiver;

// 自分が作ったクレート
use crate::common::constants::{CONFIG_FILE_PATH, SWITCH_ON, SWITCH_OFF};

// JSONファイルの構造体
#[derive(Deserialize)]
struct Config {
    switchbot_api_token: String,
    device_url_on: String,
    device_url_off: String,
}

pub struct LightControlThread {
    receiver: Receiver<u8>, 
    client: Client,
    token: String,
    device_url_on: String,
    device_url_off: String,
}

impl LightControlThread {
    pub fn new(receiver: Receiver<u8>) -> Self {
        // JSONファイルを読み込む
        let file = File::open(CONFIG_FILE_PATH).expect("Failed to open config file"); // 定数を使用
        let reader = BufReader::new(file);
        let config: Config = serde_json::from_reader(reader).expect("Failed to parse config file");

        Self {
            receiver,
            client: Client::new(),
            token: config.switchbot_api_token,
            device_url_on: config.device_url_on,
            device_url_off: config.device_url_off,
        }
    }

    // メイン処理
    pub fn run(&self) {
        while let Ok(message) = self.receiver.recv() {
            println!("Received message: {}", message);
            // メッセージに基づいてSwitchBot APIにリクエストを送信
            match message {
                SWITCH_ON => self.send_switchbot_command(&self.device_url_on),
                SWITCH_OFF => self.send_switchbot_command(&self.device_url_off),
                _ => println!("Unknown message"),
            }
        }
    }

    // SwitchBot APIにリクエストを送信
    fn send_switchbot_command(&self, device_url: &str) {
        let mut headers = HeaderMap::new();
        headers.insert(AUTHORIZATION, HeaderValue::from_str(&self.token).unwrap());
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

        let body = json!({
            "command": "turnOn",
            "parameter": "default",
            "commandType": "command"
        });

        let response = self.client.post(device_url)
            .headers(headers)
            .json(&body)
            .send();

        match response {
            Ok(_res) => println!("Switch bot response: OK"),
            Err(err) => eprintln!("Failed to send command to SwitchBot API: {:?}", err),
        }
    }
}
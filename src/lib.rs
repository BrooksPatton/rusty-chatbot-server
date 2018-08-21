mod websocket;
mod chatbot;

use std::sync::mpsc;

pub struct Config {
  twitch_token: String,
  twitch_nickname: String,
  twitch_channel_name: String,
  websocket_ip_address: String,
}

impl Config {
  pub fn new(twitch_token: String, twitch_nickname: String, twitch_channel_name: String, websocket_ip_address: String) -> Config {
    Config {
      twitch_token,
      twitch_nickname,
      twitch_channel_name,
      websocket_ip_address,
    }
  }
}

pub fn run(config: Config) {
  let (tx, rx) = mpsc::channel();
  websocket::run(tx, config.websocket_ip_address);
  chatbot::run(rx, config.twitch_token, config.twitch_nickname, config.twitch_channel_name);
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn creating_new_server_config() {
    let token = String::from("token");
    let nickname = String::from("nick");
    let channel_name = String::from("channel");
    let websocket_ip_address = String::from("127.0.0.1:3002");

    let config = Config::new(token, nickname, channel_name, websocket_ip_address);

    assert_eq!(config.twitch_token, "token");
    assert_eq!(config.twitch_nickname, "nick");
    assert_eq!(config.twitch_channel_name, "channel");
    assert_eq!(config.websocket_ip_address, "127.0.0.1:3002");
  }
}
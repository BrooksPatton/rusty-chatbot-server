extern crate rusty_chatbot_server;

use rusty_chatbot_server::run;
use rusty_chatbot_server::ServerConfig;
use std::env;

fn main() {
  let twitch_oauth_token = env::var("TWITCH_OAUTH").unwrap();
  let ip_address = "127.0.0.1:3002";
  let nickname = "brookzerker_bot";
  let channel_name = "#brookzerker";
  let config = ServerConfig::new(ip_address, twitch_oauth_token, nickname, channel_name);

  run(config);
}

extern crate rusty_chatbot_server;

use rusty_chatbot_server::Config;
use std::env;

fn main() {
  let twitch_token = env::var("TWITCH_OAUTH").expect("Twitch oauth token missing");
  let twitch_nickname = String::from("brookzerker_bot");
  let twitch_channel_name = String::from("#brookzerker");
  let websocket_ip_address = String::from("0.0.0.0:3002");
  let config = Config::new(twitch_token, twitch_nickname, twitch_channel_name, websocket_ip_address);

  rusty_chatbot_server::run(config);
}

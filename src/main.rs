extern crate rusty_chatbot_server;

use rusty_chatbot_server::Config;
use std::env;

fn main() {
  let twitch_token = env::var("TWITCH_OAUTH").unwrap();
  let twitch_nickname = String::from("brookzerker_bot");
  let twitch_channel_name = String::from("#brookzerker");
  let websocket_ip_address = String::from("127.0.0.1:3002");
  let config = Config::new(twitch_token, twitch_nickname, twitch_channel_name, websocket_ip_address);

  rusty_chatbot_server::run(config);
}

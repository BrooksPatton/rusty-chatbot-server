// extern crate ws;
// extern crate irc;

mod websocket;

// use irc::client::prelude::*;
use std::sync::mpsc;

// pub struct ServerConfig {
//   websocket_address: String,
//   twitch_oauth_token: String,
//   twitch_nickname: String,
//   twitch_channel_name: String,
// }

// impl ServerConfig {
//   pub fn new(websocket_address: &str, twitch_oauth_token: String, twitch_nickname: &str, twitch_channel_name: &str) -> ServerConfig {
//     ServerConfig {
//       websocket_address: websocket_address.to_owned(),
//       twitch_oauth_token,
//       twitch_nickname: twitch_nickname.to_owned(),
//       twitch_channel_name: twitch_channel_name.to_owned(),
//     }
//   }
// }

pub fn run() {
  let (tx, rx) = mpsc::channel();
  websocket::run(tx);
  let mut count = 0;
  let socket = rx.recv().unwrap();

  loop {
    // send the count to the client
    socket.send(count);
    count += 1;
    std::thread::sleep(std::time::Duration::from_secs(5));
  }
}

// fn generate_irc_config(config: &ServerConfig) -> Config {
//   Config {
//     nickname: Some(config.twitch_nickname.clone()),
//     server: Some("irc.chat.twitch.tv".to_owned()),
//     port: Some(6667),
//     password: Some(config.twitch_oauth_token.clone()),
//     channels: Some(vec![config.twitch_channel_name.clone()]),
//     ..Default::default()
//   }
// }

// pub fn run(config: ServerConfig) {
//   let irc_config = generate_irc_config(&config);
//   let irc_client = IrcClient::from_config(irc_config).unwrap();

//   irc_client.identify().unwrap();

//   let websocket = ws::WebSocket::new(|sender: ws::Sender| {
//     sender.send("connection established").unwrap();

//     let handler = move |message| {
//       println!("received message: {}", message);

//       sender.send(format!("You sent me {}", message))
//     };

//     handler
//   }).unwrap();

//   websocket.listen(config.websocket_address).unwrap();
// }

// #[cfg(test)]
// mod tests {
//   use super::*;

//   #[test]
//   fn create_new_config() {
//     let ip_address = "127.0.0.1:3002";
//     let twitch_oauth_token = String::from("oauth:1234");
//     let nickname = "my_nickname";
//     let channel_name = "my_channel";
//     let config = ServerConfig::new(ip_address, twitch_oauth_token, nickname, channel_name);

//     assert_eq!(config.websocket_address, "127.0.0.1:3002");
//     assert_eq!(config.twitch_oauth_token, "oauth:1234");
//     assert_eq!(config.twitch_nickname, "my_nickname");
//     assert_eq!(config.twitch_channel_name, "my_channel");
//   }
// }
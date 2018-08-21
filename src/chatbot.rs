extern crate irc;

use chatbot::irc::client::prelude::*;
use std::sync::mpsc;
use websocket::WebSocket;
use chatbot::irc::client::ext::ClientExt;

fn generate_irc_config(nickname: String, token: String, channel: String) -> Config {
  Config {
    nickname: Some(nickname.clone()),
    server: Some("irc.chat.twitch.tv".to_owned()),
    port: Some(6667),
    password: Some(token.clone()),
    channels: Some(vec![channel.clone()]),
    ..Default::default()
  }
}

pub fn run(rx: mpsc::Receiver<WebSocket>, token: String, nickname: String, channel: String) {
  let websocket = rx.recv().unwrap();
  let config = generate_irc_config(nickname, token, channel);
  let client = IrcClient::from_config(config).unwrap();

  client.identify().unwrap();

  client.send("Hello world!").unwrap();

  client.for_each_incoming(|irc_message| {
    if let Command::PRIVMSG(channel, message) = irc_message.command {
      println!("{}", message);
      client.send_privmsg(&channel, "hello from the bot").unwrap();
    }
  }).unwrap();
}
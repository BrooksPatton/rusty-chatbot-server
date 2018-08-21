extern crate irc;

use chatbot::irc::client::prelude::*;
use std::sync::mpsc;
use super::websocket::WebSocket;

fn generate_irc_config(nickname: String, token: String, channel: String) -> Config {
  Config {
    nickname: Some(nickname), // use the name of a twitch account for the bot
    server: Some("irc.chat.twitch.tv".to_owned()),
    port: Some(6667),
    password: Some(token), // use the oauth token for the bot
    channels: Some(vec![channel]),
    ..Default::default()
  }
}

pub fn run(token: String, nickname: String, channel: String, rx: mpsc::Receiver<WebSocket>) {
    let websocket = rx.recv().unwrap();
    let irc_config = generate_irc_config(nickname, token, channel);

    let irc_client = IrcClient::from_config(irc_config).unwrap();

    irc_client.identify().unwrap();

    irc_client.for_each_incoming(|raw_message| {
        let raw_message_clone = raw_message.clone();

        if let Command::PRIVMSG(channel, message) = raw_message.command {
            if let Some(nickname) = raw_message_clone.source_nickname() {
              println!("{}", message);
                if message.contains("!ping") {
                    println!("pong");

                    irc_client.send_privmsg(&channel, "pong").unwrap();
                } else if message.contains("!rustbook") {
                    irc_client.send_privmsg(&channel, "The Rust book, a great way to begin learning the language. https://doc.rust-lang.org/book/second-edition/index.html").unwrap();
                } else {
                  let response = format!("{{\"nickname\":\"{}\",\"message\":\"{}\"}}", nickname , message);

                  println!("Sending message to websocket: {}", response);
                  websocket.send(response);
                }
            }
        }
    }).unwrap();
}
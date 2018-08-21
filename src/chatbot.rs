extern crate irc;

use chatbot::irc::client::prelude::*;
use std::env;

pub fn run() {
    let oauth_token = env::var("TWITCH_OAUTH").unwrap();

    let irc_config = Config {
        nickname: Some("brookzerker_bot".to_owned()), // use the name of a twitch account for the bot
        server: Some("irc.chat.twitch.tv".to_owned()),
        port: Some(6667),
        password: Some(oauth_token), // use the oauth token for the bot
        channels: Some(vec!["#brookzerker".to_owned()]),
        ..Default::default()
    };

    let irc_client = IrcClient::from_config(irc_config).unwrap();

    irc_client.identify().unwrap();

    irc_client.for_each_incoming(|raw_message| {
        let raw_message_clone = raw_message.clone();

        if let Command::PRIVMSG(channel, message) = raw_message.command {
            if let Some(nickname) = raw_message_clone.source_nickname() {
                println!("{}: {}",nickname , message);

                if message.contains("!ping") {
                    println!("pong");

                    irc_client.send_privmsg(&channel, "pong").unwrap();
                } else if message.contains("!rustbook") {
                    irc_client.send_privmsg(&channel, "The Rust book, a great way to begin learning the language. https://doc.rust-lang.org/book/second-edition/index.html").unwrap();
                }
            }
        }
    }).unwrap();
}


// extern crate irc;

// use chatbot::irc::client::prelude::*;
// use std::sync::mpsc;
// use websocket::WebSocket;
// use chatbot::irc::client::ext::ClientExt;

// fn generate_irc_config(nickname: String, token: String, channel: String) -> Config {
//   Config {
//     nickname: Some(nickname.clone()),
//     server: Some("irc.chat.twitch.tv".to_owned()),
//     port: Some(6667),
//     password: Some(token.clone()),
//     channels: Some(vec![channel.clone()]),
//     ..Default::default()
//   }
// }

// pub fn run(rx: mpsc::Receiver<WebSocket>, token: String, nickname: String, channel: String) {
//   let websocket = rx.recv().unwrap();
//   let config = generate_irc_config(nickname, token, channel);
//   let client = IrcClient::from_config(config).unwrap();

//   client.identify().unwrap();

//   client.send("Hello world!").unwrap();

//   client.for_each_incoming(|irc_message| {
//     if let Command::PRIVMSG(channel, message) = irc_message.command {
//       println!("{}", message);
//       client.send_privmsg(&channel, "hello from the bot").unwrap();
//     }
//   }).unwrap();
// }
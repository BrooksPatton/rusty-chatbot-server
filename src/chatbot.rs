extern crate irc;

use chatbot::irc::client::prelude::*;
use std::sync::mpsc::Receiver;
use std::sync::mpsc;
use super::websocket::WebSocket;
use std::thread;
use std::time::Duration;

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

pub fn run(token: String, nickname: String, channel: String, websocket_rx: mpsc::Receiver<WebSocket>) {
    // spawn thread that will get the websocket, and check for new messages and then send them to the client
    let (chatbot_tx, chatbot_rx) = mpsc::channel();
    spawn_websocket_sender_thread(websocket_rx, chatbot_rx);

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
                    if nickname != "brookzerker_bot" {
                        let response = format!("{{\"nickname\":\"{}\",\"message\":\"{}\"}}", nickname , message);

                        println!("Sending message to websocket: {}", response);
                        chatbot_tx.send(response).expect("unable to send chat message to thread");
                    }
                }
            }
        }
    }).unwrap();
}

fn spawn_websocket_sender_thread(websocket_rx: Receiver<WebSocket>, chatbot_rx: Receiver<String>) {
    thread::spawn(move || {
        let websocket = websocket_rx.recv().expect("unable to get websocket from the rx");

        loop {
            for message in &chatbot_rx {
                websocket.send(message);
            }
            thread::sleep(Duration::from_millis(0));
        }
    });
}
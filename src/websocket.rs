extern crate ws;

use std::thread;
use std::thread::sleep;
use std::time::Duration;
use self::ws::{connect, listen, CloseCode, Sender, Handler, Message, Result};
use std::sync::mpsc;

struct Server {
  out: Sender,
}

impl Handler for Server {
  fn on_message(&mut self, message: Message) -> Result<()> {
    println!("Server got message {}", message);
    self.out.send(message)
  }
}

pub struct WebSocket {
  out: Sender,
}

impl WebSocket {
  pub fn send(&self, message: u32) {
    self.out.send(format!("{}", message)).unwrap();
  }
}

pub fn run(tx: mpsc::Sender<WebSocket>, ip_address: String) {
  let server = thread::spawn(move || {
    listen(ip_address, move |out| {
      println!("Web socket spawning");
      let websocket = WebSocket{
        out: out.clone(),
      };

      tx.send(websocket).unwrap();
      Server {out}
    }).unwrap()
  });

  sleep(Duration::from_millis(10));
}
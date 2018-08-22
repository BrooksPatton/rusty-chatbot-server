extern crate ws;

use std::thread;
use std::thread::sleep;
use std::time::Duration;
use self::ws::{listen, Sender, Handler, Message, Result};
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
  pub fn send(&self, message: String) {
    self.out.broadcast(format!("{}", message)).unwrap();
  }
}

pub fn run(tx: mpsc::Sender<WebSocket>, ip_address: String) {
  thread::spawn(move || {
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
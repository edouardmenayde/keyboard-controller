extern crate dbus;

use dbus::{Connection, BusType, Message};

const BUS_NAME: &str = "org.gnome.SettingsDaemon.Color";
const PATH: &str = "/org/gnome/SettingsDaemon/Color";
const INTERFACE: &str = "org.freedesktop.DBus.Properties";

pub struct GnomeColorSettings {
  connection: Connection
}

impl GnomeColorSettings {
  fn create_message(method: &str) -> Message {
    Message::new_method_call(BUS_NAME, PATH, INTERFACE, method).unwrap()
  }

  fn send_message(&self, message: Message) -> Message {
    self.connection.send_with_reply_and_block(message, 2000).unwrap()
  }

  pub fn new() -> GnomeColorSettings {
    GnomeColorSettings {
      connection: Connection::get_private(BusType::Session).unwrap()
    }
  }

  pub fn get(&self, property: &str) {
    println!("{}", property);
    let message = GnomeColorSettings::create_message("Get").append1(property);
    let response = self.send_message(message);

    let s: bool = response.get1().unwrap();

    println!("{}", s);
  }
}

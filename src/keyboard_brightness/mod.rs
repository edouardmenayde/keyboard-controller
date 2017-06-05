extern crate dbus;

use dbus::{Connection, BusType, Message};

const BUS_NAME: &str = "org.freedesktop.UPower";
const PATH: &str = "/org/freedesktop/UPower/KbdBacklight";
const INTERFACE: &str = "org.freedesktop.UPower.KbdBacklight";

const GET_BRIGHTNESS_METHOD: &str = "GetBrightness";
const GET_MAX_BRIGHTNESS_METHOD: &str = "GetMaxBrightness";
const SET_BRIGHTNESS_METHOD: &str = "SetBrightness";

pub struct KeyboardBrightness {
  connection: Connection
}

impl KeyboardBrightness {
  fn create_message(method: &str) -> Message {
    Message::new_method_call(BUS_NAME, PATH, INTERFACE, method).unwrap()
  }

  fn send_message(&self, message: Message) -> Message {
    self.connection.send_with_reply_and_block(message, 2000).unwrap()
  }

  pub fn new() -> KeyboardBrightness {
    KeyboardBrightness {
      connection: Connection::get_private(BusType::System).unwrap()
    }
  }

  pub fn get(&self) -> i32 {
    let message = KeyboardBrightness::create_message(GET_BRIGHTNESS_METHOD);
    let response = self.send_message(message);

    response.get1().unwrap()
  }

  pub fn get_max(&self) -> i32 {
    let message = KeyboardBrightness::create_message(GET_MAX_BRIGHTNESS_METHOD);
    let response = self.send_message(message);

    response.get1().unwrap()
  }

  pub fn set(&self, value: i32) {
    let message = KeyboardBrightness::create_message(SET_BRIGHTNESS_METHOD).append1(value);
    self.send_message(message);
  }
}

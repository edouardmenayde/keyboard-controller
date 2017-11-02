extern crate env_logger;
#[macro_use]
extern crate log;
#[macro_use]
extern crate json;
#[macro_use]

mod utils;
mod core;
mod keyboard_controller;

use keyboard_controller::Application;

fn main() {
  env_logger::init().unwrap();

  let mut application = Application::new();

  application.start();
}

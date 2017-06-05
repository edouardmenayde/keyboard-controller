extern crate dbus;
extern crate gtk;
#[macro_use]
extern crate json;

mod keyboard_brightness;
mod gnome_color_settings;
mod core;
mod utils;

use core::{Time, time, is_night_time};
use keyboard_brightness::{KeyboardBrightness};
use gnome_color_settings::{GnomeColorSettings};
use gtk::prelude::*;
use gtk::{Builder, Switch, Scale, Adjustment};
use std::f64;

const ICON_PATH: &str = "../assets/icon.png";

#[derive(Clone)]
struct Window {
  scale: Scale,
  switch: Switch,
  adjustment: Adjustment
}

#[derive(Clone)]
struct Application {
  window: Option<Window>
}

impl Application {
  fn new() -> Application {
    Application {
      window: None
    }
  }

  fn save_configuration(&self) {
    if let Some(ref window) = self.window {
      let configuration = utils::ConfigurationBuilder::new()
          .enabled(window.switch.get_state())
          .backlight_level(window.scale.get_value() as i32)
          .finalize();

      utils::save_configuration(configuration.to_json());
    }
  }

  fn restore_configuration(&self) {
    if let Some(ref window) = self.window {
      let configuration = utils::get_configuration();

      window.adjustment.set_value(configuration.backlighting_level as f64);
      window.switch.set_state(configuration.enabled);
    }
  }


  fn save_window(mut self, window: Window) {
    self.window = Some(window);
  }
}

fn launch_application(application: Application) {
  let glade_src = include_str!("main.ui");

  if gtk::init().is_err() {
    println!("Failed to initialize GTK.");
    return;
  }

  let builder = gtk::Builder::new_from_string(glade_src);

  let keyboard_brightness = KeyboardBrightness::new();

  let max = keyboard_brightness.get_max() as f64;

  let start_time = Time {
    hour: 09,
    minutes: 00
  };

  let end_time = Time {
    hour: 08,
    minutes: 00
  };

  let window: gtk::Window = builder.get_object("window").unwrap();
  let switch: Switch = builder.get_object("night-mode-switch").unwrap();
  let adjustment: Adjustment = builder.get_object("keyboard-backlight-level").unwrap();
  let scale: Scale = builder.get_object("keyboard-backlight-scale").unwrap();

  //  {
  //    let scale = scale.clone();
  //    let switch = switch.clone();
  //    let adjustment = adjustment.clone();
  //
  //    application.save_window(Window {
  //      scale: scale,
  //      switch: switch,
  //      adjustment: adjustment
  //    });
  //  }

  application.restore_configuration();

  let app = application.clone();
  scale.connect_change_value(move |scale, _, value| {
    if value.fract() > 0.5 {
      scale.set_value(value.ceil());
    } else {
      scale.set_value(value.floor());
    }

    app.save_configuration();

    gtk::Inhibit(true)
  });

  switch.connect_state_set(move |_, _| {
    application.save_configuration();

    gtk::Inhibit(false)
  });

  adjustment.set_upper(max);
  adjustment.set_lower(0f64);

  window.connect_delete_event(|_, _| {
    gtk::main_quit();

    Inhibit(false)
  });

  //  let status_icon = gtk::StatusIcon::new_from_file(ICON_PATH);
  let status_icon = gtk::StatusIcon::new_from_icon_name("keyboard");
  status_icon.set_visible(true);

  gtk::timeout_add_seconds(1, move || {
    let is_night_time_enable = switch.get_state();

    if !is_night_time_enable {
      keyboard_brightness.set(0);
    } else {
      let now = time::now();

      let is_night_time = is_night_time(&now, &start_time, &end_time);

      if is_night_time {
        keyboard_brightness.set(scale.get_value() as i32);
      } else {
        keyboard_brightness.set(0);
      }
    }

    Continue(true)
  });

  window.show_all();

  gtk::main();
}

fn main() {
  let application: Application = Application::new();

  launch_application(application.clone());
}

extern crate dbus;
extern crate gtk;
#[macro_use]
extern crate json;
#[macro_use]
extern crate log;
extern crate env_logger;
extern crate gdk_pixbuf;
extern crate time;

use std::env;
use std::path::Path;
use std::rc::Rc;
use gdk_pixbuf::Pixbuf;

mod keyboard_brightness;
mod core;
mod utils;

use utils::{ConfigurationBuilder, get_configuration, save_configuration};
use core::{Time, is_night_time};
use keyboard_brightness::KeyboardBrightness;
use gtk::prelude::*;
use gtk::{Switch, Scale, Adjustment, StatusIcon, SpinButton, CssProvider, StyleContext, Button,
          Menu, MenuItem, timeout_add_seconds, Inhibit};
use std::f64;

const ICON_PATH: &str = "assets/icon.png";
const LOWER: f64 = 0f64;

#[derive(Clone)]
struct Window {
  scale: Scale,
  switch: Switch,
  adjustment: Adjustment,
  start_hour_spin: SpinButton,
  start_minute_spin: SpinButton,
  end_hour_spin: SpinButton,
  end_minute_spin: SpinButton
}

#[derive(Clone)]
struct Application {
  window: Option<Rc<Window>>
}

fn construct_time(hour_spin: &SpinButton, minute_spin: &SpinButton) -> Time {
  Time {
    hours: hour_spin.get_value() as i32,
    minutes: minute_spin.get_value() as i32
  }
}

impl Application {
  fn new() -> Application {
    Application {
      window: None
    }
  }

  fn save_configuration(&self) {
    if let Some(ref window) = self.window {
      let configuration = ConfigurationBuilder::new()
          .enabled(window.switch.get_state())
          .backlight_level(window.scale.get_value() as i32)
          .start_time(construct_time(&window.start_hour_spin, &window.start_minute_spin))
          .end_time(construct_time(&window.end_hour_spin, &window.end_minute_spin))
          .finalize();

      save_configuration(configuration);
    }
  }

  fn restore_configuration(&self) {
    if let Some(ref window) = self.window {
      let configuration = get_configuration();

      window.adjustment.set_value(configuration.backlighting_level as f64);
      window.switch.set_state(configuration.enabled);

      window.start_hour_spin.set_value(configuration.start_time.hours as f64);
      window.start_minute_spin.set_value(configuration.start_time.minutes as f64);

      window.end_hour_spin.set_value(configuration.end_time.hours as f64);
      window.end_minute_spin.set_value(configuration.end_time.minutes as f64);
    }
  }

  fn save_window(&mut self, window: Window) {
    self.window = Some(Rc::new(window));
  }
}

fn launch_application() {
  let mut application = Application::new();

  let current_dir = env::current_dir().unwrap();

  let glade_src = include_str!("main.ui");

  if gtk::init().is_err() {
    error!("Failed to initialize GTK.");
    return;
  }

  let builder = gtk::Builder::new_from_string(glade_src);

  let css_path = current_dir.join(Path::new("src/main.css")); // Hacky...

  let css_provider = CssProvider::new();

  if css_provider.load_from_path(css_path.to_str().unwrap()).is_err() {
    error!("Unable to load css");
  }

  let keyboard_brightness = KeyboardBrightness::new();

  let upper = keyboard_brightness.get_max() as f64;

  let window: gtk::Window = builder.get_object("window").unwrap();
  let screen = window.get_screen().unwrap();

  StyleContext::add_provider_for_screen(&screen, &css_provider, gtk::STYLE_PROVIDER_PRIORITY_APPLICATION);

  let switch: Switch = builder.get_object("night-mode-switch").unwrap();
  let adjustment: Adjustment = builder.get_object("keyboard-backlight-level").unwrap();
  let scale: Scale = builder.get_object("keyboard-backlight-scale").unwrap();

  let start_hour_spin: SpinButton = builder.get_object("lighting-start-hour").unwrap();
  let start_minute_spin: SpinButton = builder.get_object("lighting-start-minute").unwrap();

  let end_hour_spin: SpinButton = builder.get_object("lighting-end-hour").unwrap();
  let end_minute_spin: SpinButton = builder.get_object("lighting-end-minute").unwrap();

  adjustment.set_upper(upper);
  adjustment.set_lower(LOWER);

  let save_button: Button = builder.get_object("save_button").unwrap();

  {
    let scale = scale.clone();
    let switch = switch.clone();
    let adjustment = adjustment.clone();
    let start_hour_spin = start_hour_spin.clone();
    let start_minute_spin = start_minute_spin.clone();
    let end_hour_spin = end_hour_spin.clone();
    let end_minute_spin = end_minute_spin.clone();

    application.save_window(Window {
      scale,
      switch,
      adjustment,
      start_hour_spin,
      start_minute_spin,
      end_hour_spin,
      end_minute_spin
    });

    application.restore_configuration();
  }

  {
    let application = application.clone();

    let save_button = save_button.clone();

    save_button.connect_clicked(move |_| {
      application.save_configuration();

      Inhibit(false);
    });
  }

  {
    let application = application.clone();

    scale.connect_change_value(move |scale, _, value| {
      if value.fract() > 0.5 {
        scale.set_value(value.ceil());
      } else {
        scale.set_value(value.floor());
      }

      application.save_configuration();

      Inhibit(true)
    });
  }

  window.connect_delete_event(|_, _| {
    gtk::main_quit();

    Inhibit(false)
  });

  let icon = Pixbuf::new_from_file(ICON_PATH).unwrap();

  let status_icon = StatusIcon::new_from_pixbuf(&icon);
  status_icon.set_visible(true);

  let popup_menu = Menu::new();
  let pause = MenuItem::new_with_label("Pause");
  let exit = MenuItem::new_with_label("Exit");

  popup_menu.append(&pause);
  popup_menu.append(&exit);

  popup_menu.show_all();

  status_icon.connect_popup_menu(move |_icon: &StatusIcon, button: u32, activate_time: u32| {
    popup_menu.popup_easy(button, activate_time);
  });

  timeout_add_seconds(1, move || {
    let start_time = construct_time(&start_hour_spin, &start_minute_spin);
    let end_time = construct_time(&end_hour_spin, &end_minute_spin);

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
  env_logger::init().unwrap();

  launch_application();
}

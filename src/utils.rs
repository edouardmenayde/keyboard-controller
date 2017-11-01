extern crate json;

use core::{Time, is_night_time};

use json::JsonValue;

use std::env;
use std::ops::Deref;
use std::path::PathBuf;
use std::io::{Write, Read};
use std::fs::{File, OpenOptions, remove_file};

const CONFIG_FILE: &str = ".config/.keyboard-controller";

const DEFAULT_START_TIME: Time = Time {
  hours: 21,
  minutes: 00
};

const DEFAULT_END_TIME: Time = Time {
  hours: 07,
  minutes: 00
};

pub struct Configuration {
  pub enabled: bool,
  pub backlighting_level: i32,
  pub start_time: Time,
  pub end_time: Time
}

impl Configuration {
  fn from_str(configuration: &str) -> Configuration {
    match json::parse(configuration) {
      Ok(configuration) => {
        Configuration::from_json(configuration)
      }
      Err(_) => {
        ConfigurationBuilder::new().finalize()
      }
    }
  }

  fn from_json(input_config: JsonValue) -> Configuration {
    let enabled = input_config["enabled"].as_bool().unwrap_or(false);
    let backlighting_level = input_config["backlighting_level"].as_i32().unwrap_or(0);
    let start_time = Time::from_string(input_config["start_time"].as_str().unwrap().to_string());
    let end_time = Time::from_string(input_config["end_time"].as_str().unwrap().to_string());

    Configuration {
      enabled,
      backlighting_level,
      start_time,
      end_time
    }
  }

  pub fn to_json(&self) -> JsonValue {
    println!("{}", self.end_time.to_string());
    object! {
      "enabled" => self.enabled,
      "backlighting_level" => self.backlighting_level,
      "start_time" => self.start_time.to_string(),
      "end_time" => self.end_time.to_string()
    }
  }
}

pub struct ConfigurationBuilder {
  pub enabled: bool,
  pub backlighting_level: i32,
  pub start_time: Time,
  pub end_time: Time
}

impl ConfigurationBuilder {
  pub fn new() -> ConfigurationBuilder {
    ConfigurationBuilder {
      enabled: false,
      backlighting_level: 0,
      start_time: DEFAULT_START_TIME.clone(),
      end_time: DEFAULT_END_TIME.clone()
    }
  }

  pub fn enabled(&mut self, enabled: bool) -> &mut ConfigurationBuilder {
    self.enabled = enabled;

    self
  }

  pub fn backlight_level(&mut self, backlighting_level: i32) -> &mut ConfigurationBuilder {
    self.backlighting_level = backlighting_level;

    self
  }

  pub fn start_time(&mut self, start_time: Time) -> &mut ConfigurationBuilder {
    self.start_time = start_time;

    self
  }

  pub fn end_time(&mut self, end_time: Time) -> &mut ConfigurationBuilder {
    self.end_time = end_time;

    self
  }

  pub fn finalize(&self) -> Configuration {
    Configuration {
      enabled: self.enabled,
      backlighting_level: self.backlighting_level,
      start_time: self.start_time,
      end_time: self.end_time
    }
  }
}

fn get_home_dir() -> PathBuf {
  if let Some(path) = env::home_dir() {
    path
  } else {
    PathBuf::from(".")
  }
}

fn get_config_path() -> PathBuf {
  get_home_dir().deref().join(CONFIG_FILE)
}

pub fn get_config_file() -> File {
  let config_path = get_config_path();

  if config_path.exists() {
    return OpenOptions::new().write(true).read(true).open(config_path).unwrap();
  }

  File::create(get_config_path()).unwrap()
}

/// Need to handle json here and bad json
pub fn get_configuration() -> Configuration {
  let mut file = get_config_file();

  let mut configuration = String::new();

  match file.read_to_string(&mut configuration) {
    Ok(_) => {
      Configuration::from_str(configuration.as_str())
    }
    Err(_) => {
      ConfigurationBuilder::new().finalize()
    }
  }
}

pub fn save_configuration(configuration: Configuration) {
  let config_path = get_config_path();
  remove_file(config_path).unwrap();

  let mut file = get_config_file();

  match file.write_all(configuration.to_json().dump().as_bytes()) {
    Ok(_) => {
      info!("Configuration saved");
      info!("{}", configuration.to_json().dump());
    }
    Err(error) => {
      error!("{}", error);
    }
  }
}

//mod tests {
//  use super::Ut;
//
//  //  mod utils {
//  //    use std::path::{PathBuf};
//  //
//  //    fn get_home_dir() -> PathBuf { PathBuf::from(".") }
//  //
//  //    const CONFIG_FILE: &str = "tests/.tmp/";
//  //  }
//
//  #[test]
//  fn test_save_configuration() {
//    save_configuration("");
//  }
//}

extern crate json;

use json::JsonValue;

use std::env;
use std::ops::Deref;
use std::path::{Path, PathBuf};
use std::io;
use std::io::{Write, Error, Read, Seek, SeekFrom};
use std::fs::{File, metadata};

const CONFIG_FILE: &str = ".config/.keyboard-controller";

pub struct Configuration {
  pub enabled: bool,
  pub backlighting_level: i32
}

impl Configuration {
  fn set_enabled() {}

  fn new() -> Configuration {
    Configuration {
      enabled: false,
      backlighting_level: 0
    }
  }

  fn from_str(configuration: &str) -> Configuration {
    match json::parse(configuration) {
      Ok(configuration) => {
        Configuration::from_json(configuration)
      }
      Err(_) => {
        Configuration::new()
      }
    }
  }

  fn from_json(input_config: JsonValue) -> Configuration {
    let enabled = input_config["enabled"].as_bool().unwrap_or(false);
    let backlighting_level = input_config["backlighting_level"].as_i32().unwrap_or(0);

    Configuration {
      enabled: enabled,
      backlighting_level: backlighting_level
    }
  }

  pub fn to_json(&self) -> JsonValue {
    object! {
      "enabled" => self.enabled,
      "backlighting_level"=> self.backlighting_level
    }
  }
}

pub struct ConfigurationBuilder   {
  pub enabled: bool,
  pub backlighting_level: i32
}

impl ConfigurationBuilder {
  pub fn new() -> ConfigurationBuilder {
    ConfigurationBuilder {
      enabled: false,
      backlighting_level: 0
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

  pub fn finalize(&self) -> Configuration {
    Configuration {
      enabled: self.enabled,
      backlighting_level: self.backlighting_level
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

pub fn create_config_file() -> File {
  let mut file = File::create(get_home_dir().deref().join(CONFIG_FILE)).unwrap();

//  let empty_json_object = Configuration::new().to_json();
//
//  empty_json_object.write(&mut file);

  file
}

pub fn do_get_config_file() -> File {
  File::open(get_home_dir().deref().join(CONFIG_FILE)).unwrap()
}

pub fn get_config_file() -> File {
    if !metadata(CONFIG_FILE).map(|m| m.is_file()).unwrap_or(false) {
      return create_config_file()
    }

  do_get_config_file()
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
      Configuration::new()
    }
  }
}

pub fn save_configuration(configuration: JsonValue) {
  let mut file = get_config_file();

  match file.write_all(configuration.dump().as_bytes()) {
    Ok(_) => {
      println!("Configuration saved");
    }
    Err(error) => {
      println!("{}", error);
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

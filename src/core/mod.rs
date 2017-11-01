extern crate time;

use time::Tm;

use std::str::FromStr;

#[derive(Copy, Clone)]
pub struct Time {
  pub hours: i32,
  pub minutes: i32
}

static DIVIDER: &'static str = ":";

impl Time {
  fn new() -> Time {
    Time {
      hours: 0,
      minutes: 0
    }
  }

  pub fn to_string(&self) -> String {
    format!("{}{}{}", self.hours, DIVIDER, self.minutes)
  }

  pub fn from_string(time: String) -> Time {
    let split_time: Vec<&str> = time.split(DIVIDER).collect();

    if split_time.len() > 2 {
      return Time::new();
    }

    let hours = i32::from_str(split_time[0]).unwrap_or(0);
    let minutes = i32::from_str(split_time[1]).unwrap_or(0);

    Time {
      hours,
      minutes
    }
  }
}


pub trait IsBetween {
  fn is_between(&self, start: &Time, end: &Time) -> bool;
}

impl IsBetween for Tm {
  fn is_between(&self, start: &Time, end: &Time) -> bool {
    if start.hours <= end.hours {
      if self.tm_hour == start.hours && self.tm_hour < end.hours {
        return self.tm_min >= start.minutes;
      }

      if self.tm_hour > start.hours && self.tm_hour == end.hours {
        return self.tm_min <= end.minutes;
      }

      return self.tm_min >= start.minutes && self.tm_min <= end.minutes;
    }

    start.hours > end.hours && (self.tm_hour >= start.hours || self.tm_hour <= end.hours)
  }
}

pub fn is_night_time(now: &Tm, start: &Time, end: &Time) -> bool {
  now.is_between(start, end)
}

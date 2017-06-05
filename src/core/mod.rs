extern crate time;

use time::{Tm};

pub struct Time {
  pub hour: i32,
  pub minutes: i32
}

pub trait IsBetween {
  fn is_between(&self, start: &Time, end: &Time) -> bool;
}

impl IsBetween for Tm {
  fn is_between(&self, start: &Time, end: &Time) -> bool {
    if start.hour <= end.hour {
      if self.tm_hour == start.hour && self.tm_hour < end.hour {
        return self.tm_min >= start.minutes
      }

      if self.tm_hour > start.hour && self.tm_hour == end.hour {
        return self.tm_min <= end.minutes
      }

      return self.tm_min >= start.minutes && self.tm_min <= end.minutes
    }

    start.hour > end.hour && (self.tm_hour >= start.hour || self.tm_hour <= end.hour)
  }
}

pub fn is_night_time(now: &Tm, start: &Time, end: &Time) -> bool {
  now.is_between(start, end)
}

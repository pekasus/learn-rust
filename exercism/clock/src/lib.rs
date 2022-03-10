use core::ops::Add;
use std::fmt;

const MINUTES_IN_A_DAY: i32 = 24 * 60;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct Clock(u16);

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let hours = self.0 / 60;
        let minutes = self.0 % 60;
        write!(f, "{:02}:{:02}", hours, minutes)
    }
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        Self::from_hours(hours) + Self::from_minutes(minutes)
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        *self + Self::from_minutes(minutes)
    }

    fn from_minutes(relative_minutes: i32) -> Self {
        let mut minutes = relative_minutes % MINUTES_IN_A_DAY;
        if minutes < 0 {
            minutes += MINUTES_IN_A_DAY;
        }
        Self(minutes as u16)
    }

    fn from_hours(relative_hours: i32) -> Self {
        Self::from_minutes(relative_hours * 60)
    }
}

impl Add<Self> for Clock {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self::from_minutes(self.0 as i32 + rhs.0 as i32)
    }
}

// ClockOrig is the original clock version implemented by me before receiving feeback from a
// mentor.
#[derive(Eq, PartialEq, Debug)]
pub struct ClockOrig {
    hours: i32,
    minutes: i32,
}

impl ClockOrig {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let mut extra_hours = minutes / 60;
        let mut minutes = minutes % 60;
        if minutes < 0 {
            minutes = minutes + 60;
            extra_hours = extra_hours - 1;
        }
        let mut hours = (hours + extra_hours) % 24;
        if hours < 0 {
            hours = hours + 24;
        }

        ClockOrig {
            hours: hours,
            minutes: minutes,
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        ClockOrig::new(self.hours, self.minutes + minutes)
    }
}

impl ToString for ClockOrig {
    fn to_string(&self) -> String {
        format!("{:02}:{:02}", self.hours, self.minutes)
    }
}

// --------------------------------------------------------------

// Clock is the new version taking into account the feedback of the mentor.
use std::fmt;

#[derive(Eq, PartialEq, Debug)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}
// Instead of using the magic numbers it might be considered to use consts with meaningful names.
const MINUTES_IN_HOUR: i32 = 60;
const HOURS_IN_DAY: i32 = 24;

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        // Collect everything into minutes
        let mut minutes = minutes + (hours * MINUTES_IN_HOUR);
        // Get hours
        let hours = minutes.div_euclid(MINUTES_IN_HOUR).rem_euclid(HOURS_IN_DAY);
        // Get remaining minutes
        minutes = minutes.rem_euclid(MINUTES_IN_HOUR);

        Clock {
            hours: hours,
            minutes: minutes,
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock::new(self.hours, self.minutes + minutes)
    }
}
// We can implement just the Display trait for Clock, which contains an implementation of to_string
// that is automatically implemented for any type which implements the Display trait. As such,
// ToString shouldn't be implemented directly: Display should be implemented instead, and you get the
// ToString implementation for free.
//
// Note that I'm using the same format call to format the data.
impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}

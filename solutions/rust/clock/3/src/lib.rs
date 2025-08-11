use std::fmt;
//use std::time::SystemTime;

const HOURS_PER_DAY: i32 = 24;
const MINS_PER_HOUR: i32 = 60;

#[derive(Debug, Eq, PartialEq)]
pub struct Clock{
    pub hours: i32,
    pub minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let mut c = Clock {
            hours,
            minutes
        };
        c.rectify();
        c
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock::new(self.hours, self.minutes + minutes)
    }

    fn rectify(&mut self) {
        self.hours += self.minutes / MINS_PER_HOUR;
        self.hours %= HOURS_PER_DAY;
        self.minutes %= MINS_PER_HOUR;
        if self.minutes < 0 {
            self.minutes += MINS_PER_HOUR;
            self.hours -= 1;
        }
        if self.hours < 0 {
            self.hours += HOURS_PER_DAY;
        }
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}
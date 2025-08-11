use std::fmt;
//use std::time::SystemTime;

#[derive(Debug, Eq, PartialEq)]
pub struct Clock{
    pub minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        Clock {
            minutes: (hours * 60 + minutes) % (60*24),
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock {
            minutes: (self.minutes + minutes) % (60*24),
        }
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let hours: i32 = self.minutes / 60;
        let minutes: i32 = self.minutes % 60;

        write!(f, "{:02}:{:02}", hours, minutes)
    }
}
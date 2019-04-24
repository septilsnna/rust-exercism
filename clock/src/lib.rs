use std::fmt;

#[derive(PartialEq)]
pub struct Clock{
    hours : i32,
    minutes : i32,
}

impl Clock {
    pub fn new(mut hours: i32, mut minutes: i32) -> Self {
        if minutes >= 60 {
            hours = hours + (minutes/60);
            minutes = minutes % 60;
        } else if minutes < 0 {
            hours = (hours + (minutes/60)) - 1;
            minutes = 60 + (minutes%60);
            if minutes >= 60 {
                hours = hours + (minutes/60);
                minutes = minutes % 60;
            }
        }
        if hours >= 24 {
            hours = hours%24;
        } else if hours < 0 {
            hours = 24 + (hours%24);
            if hours >= 24 {
                hours = hours%24;
            }
        }
        Clock{hours: hours, minutes: minutes}
    }

    pub fn add_minutes(mut self, minutes: i32) -> Self {
        self.minutes = self.minutes + minutes;
        Clock::new(self.hours, self.minutes)
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.hours < 10 {
            if self.minutes < 10 {
                write!(f, "0{}:0{}", self.hours, self.minutes)
            } else {
                write!(f, "0{}:{}", self.hours, self.minutes)
            }
        } else {
            if self.minutes < 10 {
                write!(f, "{}:0{}", self.hours, self.minutes)
            } else {
                write!(f, "{}:{}", self.hours, self.minutes)
            }
        }
    }
}

impl fmt::Debug for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       if self.hours < 10 {
            if self.minutes < 10 {
                write!(f, "0{}:0{}", self.hours, self.minutes)
            } else {
                write!(f, "0{}:{}", self.hours, self.minutes)
            }
        } else {
            if self.minutes < 10 {
                write!(f, "{}:0{}", self.hours, self.minutes)
            } else {
                write!(f, "{}:{}", self.hours, self.minutes)
            }
        }
    }
}
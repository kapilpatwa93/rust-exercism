use std::fmt;

#[derive(Debug)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(h: i32, m: i32) -> Self {
        let (mut min_hour, mut minutes) = min_to_hour(m);
        if minutes < 0 {
            minutes = 60 - m.abs() % 60;
            min_hour = if min_hour == 0 { -1 } else { min_hour - 1 };
        }
        let mut hours = (h + min_hour) % 24;
        if hours < 0 {
            hours = 24 - ((h + min_hour).abs() % 24);
        }
        Clock { hours, minutes }
    }
    pub fn add_minutes(&self, minutes: i32) -> Self {
        let (hour_min, min) = min_to_hour(minutes);
        Clock::new(self.hours + hour_min, self.minutes + min)
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}

fn min_to_hour(minutes: i32) -> (i32, i32) {
    (minutes / 60, minutes % 60)
}

impl PartialEq for Clock {
    fn eq(&self, other: &Self) -> bool {
        self.to_string().eq(&other.to_string())
    }
}

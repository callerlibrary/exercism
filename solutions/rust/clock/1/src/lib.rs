#[derive(PartialEq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

const DAY_MINUTES: i32 = 24 * 60;

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let total_min = hours * 60 + minutes;
        let normalized_min = (total_min % DAY_MINUTES + DAY_MINUTES) % DAY_MINUTES;
        Self {
            hours: normalized_min / 60,
            minutes: normalized_min % 60,
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let total_min = self.hours * 60 + self.minutes + minutes;
        let normalized_min = (total_min % DAY_MINUTES + DAY_MINUTES) % DAY_MINUTES;
        Self {
            hours: normalized_min / 60,
            minutes: normalized_min % 60,
        }
    }
}

impl std::fmt::Display for Clock {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}


struct Clock {
    hours: i32,
    minutes: i32,
}
const MINUTES_IN_HOUR: i32 = 60;
impl Clock {
    fn new(hours: i32, minutes: i32) -> Self {
        Self {
            hours,
            minutes,
        }
    }

    fn add_minutes(&self, minutes: i32) -> Self {
        let minutes = self.minutes + minutes;
        let new_hour = self.hours + minutes / MINUTES_IN_HOUR;
        let new_minutes = minutes % MINUTES_IN_HOUR;

        Self{
            hours: new_hour,
            minutes: new_minutes,
        }
    }

    fn to_string(&self) -> String {
        let mut time_as_string: String = self.hours.to_string() + ":" + &self.minutes.to_string();
        if time_as_string.len() != 5{
            time_as_string.insert(0, '0');
        }
        time_as_string
    }
}

fn main() {
    let clock = Clock::new(0, 45).add_minutes(160);

    assert_eq!(clock.to_string(), "03:25");
}

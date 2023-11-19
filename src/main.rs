#[derive(Debug, Eq)]
struct Clock {
    hours: i32,
    minutes: i32,
}
const MINUTES_IN_HOUR: i32 = 60;
const HOURS_IN_DAY : i32 = 24;
impl Clock {
    fn new(hours: i32, minutes: i32) -> Self {
        let mut new_clock : Clock = Clock{
            hours : 0,
            minutes : 0,
        };
        new_clock = new_clock.add_minutes(minutes);
        
        if new_clock.hours + hours >= HOURS_IN_DAY
        {
            new_clock.hours = (new_clock.hours + hours) +HOURS_IN_DAY;
        } else {
            new_clock.hours = new_clock.hours + hours;
        }

        new_clock
    }

    fn add_minutes(&self, minutes: i32) -> Self {
        let minutes = self.minutes + minutes;
        let new_minutes;
        if minutes >= MINUTES_IN_HOUR
        {
            new_minutes = minutes % MINUTES_IN_HOUR;
        } else {
            new_minutes = minutes;
        }

        let new_hour = self.hours + (minutes / MINUTES_IN_HOUR) % HOURS_IN_DAY;
        
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

impl PartialEq for Clock {
    fn eq(&self, other: &Self) -> bool {
        self.hours == other.hours && self.minutes == other.minutes
    }
}

fn main() {
    let clock = Clock::new(0, 45).add_minutes(160);
    let clock2 = Clock::new(3, 25);

    assert_eq!(clock.to_string(), "03:25");
    assert_eq!(clock, clock2);

    let clock = Clock::new(5, 32).add_minutes(1500);
    assert_eq!(clock.to_string(), "06:32");

    assert_eq!(Clock::new(2, 4322), Clock::new(2, 2));

    assert_eq!(Clock::new(3, -20), Clock::new(2, 40));

    assert_eq!(Clock::new(5, -1490), Clock::new(4, 10));
}

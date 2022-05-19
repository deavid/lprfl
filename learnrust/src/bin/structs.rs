use std::fmt::Display;

#[derive(Debug)]
pub struct Time {
    // members:
    pub hours: i64,
    pub minutes: i64,
    pub seconds: i64,
}

// Variables vs Structs  --> wrong
// Bease Types vs structs (custom types/ composite types)

impl Time {
    // Associated functions to type Time: (methods)

    // Time::new
    pub fn new(hours: i64, minutes: i64, seconds: i64) -> Self {
        Time {
            hours,
            minutes,
            seconds,
        }
    }
    // t1.add_seconds ...
    pub fn add_seconds(&mut self, secs: i64) {
        self.seconds += secs;
        self.fix();
    }

    pub fn fix(&mut self) {
        if self.seconds > 60 {
            self.minutes += self.seconds / 60;
            self.seconds %= 60;
        }
        if self.minutes > 60 {
            self.hours += self.minutes / 60;
            self.minutes %= 60;
        }
    }

    pub fn add(&self, other: &Self) -> Self {
        let mut ret = Self {
            hours: self.hours + other.hours,
            minutes: self.minutes + other.minutes,
            seconds: self.seconds + other.seconds,
        };
        ret.fix();
        ret
    }
}

impl Display for Time {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:02}:{:02}:{:02}",
            self.hours, self.minutes, self.seconds
        )
    }
}

fn main() {
    // use chrono::prelude::*;
    // let utc: DateTime<Utc> = Utc::now();
    // dbg!(utc);

    // let year = 2022;
    // let month = 5;
    // let day = 12;

    // println!("{}-{:02}-{:02}", year, month, day);

    // let mut t1 = Time::new(2, 12, 15);
    // // dbg!(t1.hours, t1.minutes, t1.seconds);
    // dbg!(&t1);
    // t1.add_seconds(2000); // t1.*** -> self

    // println!("{:?}", &t1); // {:?} -> Debug format -> for devs

    // t1.add_seconds(31000);
    // println!("{:?}", &t1); // {:?} -> Debug format -> for devs

    // let t2 = Time::new(1, 50, 50);
    // dbg!(&t2);

    // let t3 = t1.add(&t2);
    // dbg!(&t3);

    // println!("{}", t1);

    let t = Time::new(2, 12, 15); // main is owner of 't'
                                  // dbg!(&t);
                                  // let _a = &t; // t gets moved into a; t no longer exists. Other languages instead of move, they copy.
    let secs = seconds(&t); // t ownership is transferred to 'seconds()'
                            // dbg!(&t);
    dbg!(&t, secs);
}

// Rust, variables get moved instead copied.

fn seconds(t: &Time) -> i64 {
    // &Time -> borrowed for reading
    // << comes here
    t.hours * 60 * 60 + t.minutes * 60 + t.seconds
    // and 't' is destroyed (freed, memory returned to OS) here!
}


#[derive(Debug, PartialEq, Eq)]
pub struct Clock {
    hours : i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        //todo!("Construct a new Clock from {hours} hours and {minutes} minutes");
            let mut clock: Clock = Clock { hours, minutes };
            clock.hours %= 24;
            if clock.hours <0 {
                clock.hours += 24;
            }
            clock.minutes += clock.hours * 60;
            clock.minutes %= 24 * 60;
            if clock.minutes <0 {
                clock.minutes += 24 * 60;
            }
            clock.hours = clock.minutes / 60;
            clock.minutes %= 60;
            clock
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        //todo!("Add {minutes} minutes to existing Clock time");
        let mut clock: Clock = Clock { hours: self.hours, minutes: self.minutes };
        if minutes > 0 {
            clock.minutes += minutes;
            clock.hours += clock.minutes / 60;
            clock.minutes %= 60;
            clock.hours %= 24;
            clock
        } else {
            self.substract_minutes(- minutes)
        }
    }
    pub fn substract_minutes(&self, minutes: i32) -> Self {
        //todo!("Add {minutes} minutes to existing Clock time");
        //self.add_minutes(- minutes)
        let mut clock: Clock = Clock { hours: self.hours, minutes: self.minutes };
        if minutes > 0 {
            clock.minutes -= minutes;
            //println!("substract1 {}", clock);
            if clock.minutes < 0 {
                // if minutes are in [-60;0[, hour is -1, if minutes are in [-120;-60[, hour is -2, etc
                clock.hours -= 1 + (- clock.minutes) / 60;
                //println!("substract2 {}", clock);
                clock.minutes = 60 - (- clock.minutes) % 60;
                if clock.minutes == 60 {
                    clock.minutes = 0;
                    clock.hours += 1;
                }
                //println!("substract3 {}", clock);
            }
            if clock.hours < 0 {
                // if hours are in [-INF;0[, hours is 24 - hours % 24
                clock.hours = 24 - (- clock.hours) % 24;
                //println!("substract4 {}", clock);
            }
            clock
        } else {
            self.add_minutes(- minutes)
        }
}
}

use std::fmt;
// To use the `{}` marker, the trait `fmt::Display` must be implemented
// manually for the type.
impl fmt::Display for Clock {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        //write!(f, "{0:02}h{1:02}mn", self.hours, self.minutes)
        write!(f, "{0:02}:{1:02}", self.hours, self.minutes)
    }
}
fn main() {
    println!("Clock Exercice");
    let mut c = Clock::new(23, 0);
    println!("Clock: {}", c);
    c = c.add_minutes(150);
    println!("Clock: {}", c);
    //c = c.add_minutes(-150);
    c = c.substract_minutes(150);
    println!("Clock: {}", c);
    let mut c = Clock::new(16, 59);
    println!("Clock: {}", c);
    c = c.add_minutes(1022);
    println!("Clock: {}", c);
    //c = c.add_minutes(-150);
    c = c.substract_minutes(1022);
    println!("Clock: {}", c);
    let mut c = Clock::new(0, 21);
    println!("Clock: {}", c);
    c = c.add_minutes(150);
    println!("Clock: {}", c);
    //c = c.add_minutes(-150);
    c = c.substract_minutes(150);
    println!("Clock: {}", c);
    let mut c = Clock::new(-2, -20);
    println!("Clock: {}", c);
    c = c.add_minutes(150);
    println!("Clock: {}", c);
    //c = c.add_minutes(-150);
    c = c.substract_minutes(150);
    println!("Clock: {}", c);

}

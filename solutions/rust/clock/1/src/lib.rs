use std::fmt;

#[derive(Debug, PartialEq, Eq)]
pub struct Clock {
    hours : i32,
    minutes : i32
}

pub fn to_min(hrs : i32) -> i32{
    hrs * 60 
}
pub fn to_time(min : i32) -> (i32,i32){
    (min / 60 , min % 60)
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let mut time = to_min(hours) + minutes;
        time = time.rem_euclid(1440);
        let (hrs,mins) = to_time(time);
        Self { hours : hrs, minutes : mins }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Self::new(self.hours, self.minutes + minutes)
    }
}

impl fmt::Display for Clock{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }

}
#[derive(Debug)]
pub struct Time {
    year: u16,
    month: u8,
    day: u8,
    hour: u8,
    minute: u8,
    minutes: u64,
}

impl Time {
    pub fn new() -> Self {
        Time {
            year: 0,
            month: 1,
            day: 1,
            hour: 0,
            minute: 0,
            minutes: 0
        }
    }

    /*pub fn year(&self) -> u16 {
        self.year
    }

    pub fn month(&self) -> u8 {
        self.month
    }

    pub fn day(&self) -> u8 {
        self.day
    }

    pub fn hour(&self) -> u8 {
        self.hour
    }

    pub fn minute(&self) -> u8 {
        self.minute
    }*/

    pub fn minutes(&self) -> u64 {
        self.minutes
    }
}

impl Time {
    fn is_year_leap(&self) -> bool {
        (self.year % 4 == 0 && self.year % 100 != 0) || self.year % 400 == 0
    }

    fn days_in_month(&self) -> u8 {
        match self.month {
            1 => 31,
            2 => if self.is_year_leap() { 29 } else { 28 },
            3 => 31,
            4 => 30,
            5 => 31,
            6 => 30,
            7 => 31,
            8 => 31,
            9 => 30,
            10 => 31,
            11 => 30,
            12 => 31,
            _ => panic!("There is only 12 months")
        }
    }

    /*fn days_in_year(&self) -> u16 {
        if self.is_year_leap() { 366 } else { 365 }
    }*/
    
    pub fn add_minute(&mut self) {
        self.minutes += 1;
        self.minute += 1;
        
        if self.minute != 60 {
            return;
        }
        
        self.hour += 1;
        self.minute = 0;
        
        if self.hour != 24 {
            return;
        }
        
        self.day += 1;
        self.hour = 0;
        
        if self.day <= self.days_in_month() {
            return;
        }
        
        self.month += 1;
        self.day = 1;
        
        if self.month <= 12 {
            return;
        }
        
        self.month = 1;
        self.year += 1;
    }
}
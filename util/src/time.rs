use std::fmt::{Display, Formatter};

pub struct Time {
    year: u16,
    month: u8,
    day: u8,
    hour: u8,
    minute: u8,
    second: u8,
}

impl Time {
    pub fn new() -> Self {
        Time {
            year: 1,
            month: 1,
            day: 1,
            hour: 0,
            minute: 0,
            second: 0,
        }
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

    pub fn add_seconds(&mut self, seconds: u16) {
        let current_summary_seconds = seconds + self.second as u16;

        self.second = (current_summary_seconds % 60) as u8;

        if current_summary_seconds < 60 {
            return;
        }

        let current_summary_minutes = self.minute as u16 + current_summary_seconds / 60;

        self.minute = (current_summary_minutes % 60) as u8;

        if current_summary_minutes < 60 {
            return;
        }

        let current_summary_hours = self.hour as u16 + current_summary_minutes / 60;

        self.hour = (current_summary_hours % 24) as u8;

        if current_summary_hours < 24 {
            return;
        }

        let current_summary_days = self.day as u16 + current_summary_hours / 24;

        let days_in_month = self.days_in_month();
        
        self.day = (current_summary_days % (days_in_month + 1) as u16) as u8;

        if current_summary_days as u8 <= days_in_month {
            return;
        }

        if self.month < 12 {
            self.month += 1;
            return;
        }

        self.month = 1;
        self.year += 1;
    }
}

impl Display for Time {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            fmt,
            "{:02}.{:02}.{:04} {:02}:{:02}:{:02}", 
            self.day, self.month, self.year, self.hour, self.minute, self.second,
        )
    }
}
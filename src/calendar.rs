use std::io::{ stdout, stdin, Write };
use std::process::Command;

pub struct Calendar {
    day: Option<u8>,
    month: Option<u8>,
    year: Option<u64>
}

impl Calendar {
    pub fn new() -> Self {
        Self {
            day: None,
            month: None,
            year: None
        } // end initialization of Calendar
    } // end new

    pub fn set_a_valid_year(&mut self) {
        let mut input = String::new();

        loop {
            print!("\n'Enter' for default (year 1), or any number\n");
            print!(">>> "); stdout().flush().expect("write!");
            input.clear();
            stdin().read_line(&mut input).expect("read!");
            clear_terminal();

            match input.as_str() {
                "\n" => {self.year = Some(1); break}
                _ => (),
            }

            self.year = match input.trim().parse::<u64>() {
                Ok(year) => {
                    if year >= 1 {
                        Some(year)
                    } else {print!("Invalid year"); continue}
                },
                Err(_) => {print!("Invalid year"); continue}
            };
            break;
        } // end read loop
    } // end set a valid year

    pub fn set_a_valid_month(&mut self) {
        let mut input = String::new();

        loop {
            print!("\n'Enter' for default (month 1), or any number\n");
            print!(">>> "); stdout().flush().expect("write!");
            input.clear();
            stdin().read_line(&mut input).expect("read!");
            clear_terminal();

            match input.as_str() {
                "\n" => {self.month = Some(1); break}
                _ => (),
            }

            self.month = match input.trim().parse::<u8>() {
                Ok(month) => {
                    if month >= 1  && month <= 12 {
                        Some(month)
                    } else {print!("Invalid month"); continue}
                },
                Err(_) => {print!("Invalid month"); continue}
            };
            break;
        } // end read loop
    } // end set a valid month

    pub fn set_a_valid_day(&mut self) {
        if self.year != None && self.month != None {
            let mut input = String::new();

            loop {
                print!("\n'Enter' for default (day 1), or any number\n");
                print!(">>> "); stdout().flush().expect("write!");
                input.clear();
                stdin().read_line(&mut input).expect("read!");
                clear_terminal();

                match input.as_str() {
                    "\n" => {self.day = Some(1); break}
                    _ => (),
                }

                self.day = match input.trim().parse::<u8>() {
                    Ok(day) => {
                        if day >= 1  && day <= self.maximum_days_for_this_month().unwrap_or(u8::MAX) {
                            Some(day)
                        } else {print!("Invalid day"); continue}
                    },
                    Err(_) => {print!("Invalid day"); continue}
                };
                break;
            } // end read loop
        } else { print!("To set a day, month and year must be initialized first\n"); }
    } // end set a valid day

    pub fn everything_is_ok(&self) -> bool {
        let mut result: bool = true;
        if self.day == None {
            print!("Day unitialized!\n");
            result = false;
        }
        if self.month == None {
            print!("Month unitialized!\n");
            result = false;
        }
        if self.year == None {
            print!("Year unitialized!\n");
            result = false;
        }
        result
    } // end everything is ok

    pub fn print(&self) {
        if self.everything_is_ok() {
            print!("{0:>0minimum_width$}/",
                self.month.unwrap(),
                minimum_width = 2
            ); stdout().flush().expect("write!");

            print!("{0:>0minimum_width$}/",
                self.day.unwrap(),
                minimum_width = 2
            ); stdout().flush().expect("write!");

            print!("{0:>0minimum_width$}",
                self.year.unwrap(),
                minimum_width = 2
            ); stdout().flush().expect("write!");
        } // verify if everything is ok
    } // end print

    pub fn pass_one_day(&mut self) {
        if self.everything_is_ok() {
            if self.day.unwrap() < self.maximum_days_for_this_month().unwrap_or(u8::MAX) {
                self.day = Some(self.day.unwrap() + 1);
            } else {
                self.day = Some(1);

                if self.month.unwrap() < 12 {
                    self.month = Some(self.month.unwrap() + 1);
                } else {
                    self.month = Some(1);
                    self.year = Some(self.year.unwrap() + 1);
            }}
        } // verify if everything is ok
    } // end pass one day

    fn maximum_days_for_this_month(&self) -> Option<u8> {
        let mut maximum_days: Option<u8> = None;
        if self.month != None && self.year != None {
            maximum_days = match self.month.unwrap() {
                1 | 3 | 5 | 7 | 8 | 10 | 12 => Some(31),
                4 | 6 | 9 | 11 => Some(30),
                2 => {if self.its_in_a_leap_year() { Some(29) } else { Some(28) }}
                _ => {print!("The given month isn't valid!"); None}
            };
        } // verify if everything is ok
        maximum_days
    } // end maximum days for this month

    fn its_in_a_leap_year(&self) -> bool {
        if self.year.unwrap() < 100 { (self.year.unwrap() % 4) == 0 }
        else {
            ((self.year.unwrap() % 4) == 0) ||
            ( ((self.year.unwrap() % 100) == 0) &&
              ((self.year.unwrap() % 400) == 0) )
        } // end else
    } // end its in a leap year
}

fn clear_terminal() {
    let mut call = if cfg!(target_os = "windows") {
        Command::new("cls")
    } else { Command::new("clear") };
    call.status().expect("syscall!");
}

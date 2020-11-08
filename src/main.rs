mod calendar;
mod clock;

use std::{ thread, time };
use std::io::{ stdout, Write };
use std::process::Command;

fn main() {
    clear_terminal();

    let mut my_calendar = calendar::Calendar::new();
    let mut my_clock = clock::Clock::new();

    my_calendar.set_a_valid_year();
    my_calendar.set_a_valid_month();
    my_calendar.set_a_valid_day();

    my_clock.set_a_valid_hour();
    my_clock.set_a_valid_minute();
    my_clock.set_a_valid_second();

    if my_clock.everything_is_ok() & my_calendar.everything_is_ok() {
        let one_sec = time::Duration::from_millis(1000);
        loop {
            clear_terminal();
            print!("\n\t");

            my_calendar.print();
            print!(" - "); stdout().flush().expect("write!");
            my_clock.print();

            print!("\n");
            my_clock.pass_one_second();
            if my_clock.has_completed_an_entire_day() {
                my_calendar.pass_one_day();
            }
            thread::sleep(one_sec);
        }
    }
}

fn clear_terminal() {
    let mut call = if cfg!(target_os = "windows") {
        Command::new("cls")
    } else { Command::new("clear") };
    call.status().expect("syscall!");
}

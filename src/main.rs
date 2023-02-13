use std::fs::File;
use std::io::{self, Write};

use chrono::{Datelike, Weekday};

const VERTICAL_LINE: &str = "|";
const EOL: &str = "\n";
const SPACE: &str = " ";
const WEEK_DAYS: &str =
    "...Sunday.....Monday....Tuesday...Wednesday...Thursday....Friday....Saturday..\n";
const WEEK_SEPARATOR: &str =
    "+----------+----------+----------+----------+----------+----------+----------+\n";
const BLANK_ROW: &str =
    "|          |          |          |          |          |          |          |\n";

fn read_year() -> i32 {
    let mut year = String::new();
    println!("Enter the year: ");
    io::stdin().read_line(&mut year).unwrap();
    year.trim().parse().unwrap()
}

fn read_month() -> u32 {
    let mut month = String::new();
    println!("Enter the month: ");
    io::stdin().read_line(&mut month).unwrap();
    month.trim().parse().unwrap()
}

fn main() {
    let year = read_year();
    let month = read_month();

    let calendar = Calendar::new(year, month);

    calendar.write_to_file();

    // println!("{}", calendar.cal_text);
}

struct Month;

impl Month {
    fn name(month: u32) -> &'static str {
        match month {
            1 => "January",
            2 => "February",
            3 => "March",
            4 => "April",
            5 => "May",
            6 => "June",
            7 => "July",
            8 => "August",
            9 => "September",
            10 => "October",
            11 => "November",
            12 => "December",
            _ => panic!("Invalid month"),
        }
    }
}

struct Day {
    number: u32,
    // week_day: Weekday,
}

struct Week {
    days: Vec<Day>,
}

impl Week {
    fn to_string(&self) -> String {
        let mut day_number_row = String::new();

        for day in &self.days {
            day_number_row.push_str(VERTICAL_LINE);
            day_number_row.push_str(&format!("{:>2}", day.number));
            day_number_row.push_str(&SPACE.repeat(8));
        }

        day_number_row.push_str(VERTICAL_LINE);
        day_number_row.push_str(EOL);

        for _ in 0..3 {
            let blank_row = String::from(BLANK_ROW);
            day_number_row.push_str(&blank_row);
        }
        return day_number_row;
    }
}

struct Calendar {
    year: i32,
    month: u32,
    weeks: Vec<Week>,
}

impl Calendar {
    fn new(year: i32, month: u32) -> Calendar {
        let mut current_date = chrono::NaiveDate::from_ymd_opt(year, month, 1).unwrap();
        let mut weeks = Vec::new();

        while current_date.weekday() != chrono::Weekday::Sun {
            current_date = current_date - chrono::Duration::days(1);
        }

        loop {
            let mut days = Vec::new();
            for _ in 0..7 {
                days.push(Day {
                    number: current_date.day(),
                });
                current_date = current_date + chrono::Duration::days(1);
            }

            weeks.push(Week { days });

            if current_date.month() != month {
                break;
            }
        }

        Calendar { year, month, weeks }
    }

    fn to_string(&self) -> String {
        let mut cal_text = String::new();

        cal_text.push_str(&format!(
            "{} {} {}",
            SPACE.repeat(34),
            Month::name(self.month),
            self.year
        ));
        cal_text.push_str(EOL);
        cal_text.push_str(WEEK_DAYS);

        for week in &self.weeks {
            cal_text.push_str(WEEK_SEPARATOR);
            cal_text.push_str(&week.to_string());
        }

        cal_text.push_str(WEEK_SEPARATOR);

        return cal_text;
    }

    fn write_to_file(&self) {
        let file_name = format!("calendar_{}_{}.txt", self.year, self.month);
        let mut file = File::create(file_name).unwrap();
        file.write_all(self.to_string().as_bytes()).unwrap();
    }
}

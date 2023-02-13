use std::fs::File;
use std::io::{self, Write};

use chrono::Datelike;

fn main() {
    let year = read_year();
    let month = read_month();

    let calendar = Calendar::new(year, month);

    calendar.write_to_file();

    println!("{}", calendar.cal_text);
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

struct Calendar {
    year: i32,
    month: u32,
    cal_text: String,
}

impl Calendar {
    fn new(year: i32, month: u32) -> Calendar {
        let cal_text = Calendar::make_calendar(year, month);

        Calendar {
            year,
            month,
            cal_text,
        }
    }

    fn make_calendar(year: i32, month: u32) -> String {
        const VERTICAL_LINE: &str = "|";
        const EOL: &str = "\n";
        const SPACE: &str = " ";
        const WEEK_DAYS: &str =
            "...Sunday.....Monday....Tuesday...Wednesday...Thursday....Friday....Saturday..\n";
        const WEEK_SEPARATOR: &str =
            "+----------+----------+----------+----------+----------+----------+----------+\n";
        const BLANK_ROW: &str =
            "|          |          |          |          |          |          |          |\n";

        let mut cal_text = String::new();

        cal_text.push_str(&format!(
            "{} {} {}",
            SPACE.repeat(34),
            Month::name(month),
            year
        ));
        cal_text.push_str(EOL);
        cal_text.push_str(WEEK_DAYS);

        let mut current_date = chrono::NaiveDate::from_ymd_opt(year, month, 1).unwrap();

        while current_date.weekday() != chrono::Weekday::Sun {
            current_date = current_date - chrono::Duration::days(1);
        }

        loop {
            cal_text.push_str(WEEK_SEPARATOR);

            let mut day_number_row = String::new();

            for _ in 0..7 {
                day_number_row.push_str(VERTICAL_LINE);
                day_number_row.push_str(&format!("{:>2}", current_date.day()));
                day_number_row.push_str(&SPACE.repeat(8));
                current_date = current_date + chrono::Duration::days(1);
            }

            day_number_row.push_str(VERTICAL_LINE);
            day_number_row.push_str(EOL);
            cal_text.push_str(&day_number_row);

            for _ in 0..3 {
                cal_text.push_str(BLANK_ROW);
            }

            if current_date.month() != month {
                break;
            }
        }

        cal_text.push_str(WEEK_SEPARATOR);

        return cal_text;
    }

    fn write_to_file(&self) {
        let file_name = format!("calendar_{}_{}.txt", self.year, self.month);
        let mut file = File::create(file_name).unwrap();
        file.write_all(self.cal_text.as_bytes()).unwrap();
    }
}

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

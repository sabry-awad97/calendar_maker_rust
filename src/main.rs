use std::fs::File;
use std::io::{self, Write};

use chrono::Datelike;

const VERTICAL_LINE: &str = "|";
const EOL: &str = "\n";
const SPACE: &str = " ";
const WEEK_DAYS: &str =
    "...Sunday.....Monday....Tuesday...Wednesday...Thursday....Friday....Saturday..\n";
const WEEK_SEPARATOR: &str =
    "+----------+----------+----------+----------+----------+----------+----------+\n";
const BLANK_ROW: &str =
    "|          |          |          |          |          |          |          |\n";

fn make_calendar(year: i32, month: u32) -> String {
    let mut cal_text = String::new();

    cal_text.push_str(&format!(
        "{} {} {}",
        SPACE.repeat(34),
        month_name(month),
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

fn main() {
    let year = read_year();
    let month = read_month();

    let cal_text = make_calendar(year, month);
    println!("{}", cal_text);

    let file_name = format!("calendar_{}_{}.txt", year, month);
    let mut file = File::create(file_name).unwrap();
    file.write_all(cal_text.as_bytes()).unwrap();
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

fn month_name(month: u32) -> &'static str {
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

// fn first_day_of_month(year: i32, month: i32) -> i32 {
//     let y = (year - 1) as i64;
//     let m = month as i64;
//     let d = 1;

//     let yy = y + y / 4 - y / 100 + y / 400;
//     let mm = (m + 12 * ((14 - m) / 12) - 2) % 12;
//     let dd = (d + yy + (31 * mm) / 12) % 7;

//     dd as i32
// }

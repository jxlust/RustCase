use chrono::{DateTime, Local, NaiveDate, NaiveDateTime, NaiveTime};

fn main() {
    let now = Instant::now();
    let date = NaiveDate::from_ymd_opt(2024, 1, 23);
    let time = NaiveTime::from_hms_opt(10, 50, 20);
    let date_time = NaiveDateTime::new(date.unwrap(), time.unwrap());
    println!("DateTime: {}", date_time);
    let now_naive = Local::now().naive_local();
    println!("Now: {}", now_naive);
}

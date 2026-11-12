use time::{Date, Duration, Month, PrimitiveDateTime, Time};

fn main() {
    let date = Date::from_calendar_date(2026, Month::September, 18).unwrap();
    let time = Time::from_hms(14, 30, 0).unwrap();
    let start = PrimitiveDateTime::new(date, time);
    println!("Start: {start}");
    let later = add_seconds(start);
    println!("Later: {later}");
}

fn add_seconds(start: PrimitiveDateTime) -> PrimitiveDateTime {
    const GIGA: i64 = 1_000_000_000;
    let later = start + Duration::seconds(GIGA);
    later
}

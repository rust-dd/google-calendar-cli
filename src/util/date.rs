use chrono_tz::Tz;
use google_calendar3::chrono::{
    DateTime, Datelike, Duration, Local, NaiveDateTime, NaiveTime, TimeZone, Utc,
};

/// Returns the start of the current week as a `DateTime<Local>`.
///
/// This function calculates the start of the week based on the current local
/// time. The week starts on Monday.
///
/// # Examples
///
/// ```
/// let start_of_week = get_start_of_the_week();
/// println!("Start of the week: {}", start_of_week);
/// ```
pub fn get_start_of_the_week() -> DateTime<Local> {
    let now = Local::now();
    let days_to_subtract = now.weekday().num_days_from_monday() as i64;
    let start_of_the_week = now - Duration::days(days_to_subtract);
    start_of_the_week
}

/// Returns a vector of strings containing the names of the days of the week in English.
///
/// This function generates a vector where each element is the name of a day of the week,
/// starting from Monday to Sunday.
///
/// # Examples
///
/// ```
/// let days = days_in_english();
/// assert_eq!(days, vec![
///     "Monday".to_string(),
///     "Tuesday".to_string(),
///     "Wednesday".to_string(),
///     "Thursday".to_string(),
///     "Friday".to_string(),
///     "Saturday".to_string(),
///     "Sunday".to_string()
/// ]);
/// ```
pub fn days_in_english() -> Vec<String> {
    const DAYS: [&str; 7] = [
        "Monday",
        "Tuesday",
        "Wednesday",
        "Thursday",
        "Friday",
        "Saturday",
        "Sunday",
    ];

    DAYS.iter().map(|&day| day.to_string()).collect()
}

/// Converts a date string to a `DateTime<Utc>` based on the provided timezone.
///
/// This function accepts a date string that can be either in the format `HH:MM`
/// or `YYYY-MM-DD HH:MM`. It will parse the string and convert it to a `DateTime<Utc>`
/// considering the given timezone. If the string is in the `HH:MM` format, it will
/// use the current date combined with the provided time.
///
/// # Arguments
///
/// * `tz` - A timezone from the `chrono_tz` crate.
/// * `date` - A string slice that holds the date to be parsed.
///
/// # Returns
///
/// A `DateTime<Utc>` representing the parsed date and time in UTC.
///
/// # Examples
///
/// ```
/// use chrono_tz::Tz;
///
/// let tz: Tz = "America/New_York".parse().unwrap();
/// let date_str = String::from("2024-07-27 15:30");
/// let utc_date = get_date_from_string(tz, &date_str);
/// println!("{}", utc_date); // Outputs the parsed date and time in UTC
///
/// let time_str = String::from("15:30");
/// let utc_time = get_date_from_string(tz, &time_str);
/// println!("{}", utc_time); // Outputs the current date with the provided time in UTC
/// ```
pub fn get_date_from_string(tz: Tz, date: &String) -> DateTime<Utc> {
    if let Ok(parsed_time) = NaiveTime::parse_from_str(date, "%H:%M") {
        let current_date = Utc::now().date_naive();
        let combined_naive = NaiveDateTime::new(current_date, parsed_time);
        let event_date_with_timezone = tz
            .from_local_datetime(&combined_naive)
            .unwrap()
            .naive_utc()
            .and_utc();
        return event_date_with_timezone;
    } else {
        let parsed_time = NaiveDateTime::parse_from_str(date, "%Y-%m-%d %H:%M");
        let combined_naive = parsed_time.unwrap();
        let event_date_with_timezone = tz
            .from_local_datetime(&combined_naive)
            .unwrap()
            .naive_utc()
            .and_utc();
        return event_date_with_timezone;
    }
}

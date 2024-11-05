use chrono::{DateTime, Datelike, Duration, Local, NaiveDateTime, NaiveTime, Utc, TimeZone};
use chrono_tz::Tz;

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
pub fn days_in_english() -> [&'static str; 7] {
    let days = [
        "Monday",
        "Tuesday",
        "Wednesday",
        "Thursday",
        "Friday",
        "Saturday",
        "Sunday",
    ];

    days
}

/// Converts a date string to a `DateTime<Utc>` based on the provided timezone.
///
/// This function accepts a date string that can be either in the format `HH:MM`,
/// `YYYY-MM-DD HH:MM`, or `MM-DD HH:MM`. It will parse the string and convert it to a
/// `DateTime<Utc>` considering the given timezone. If the string is in the `HH:MM` format,
/// it will use the current date combined with the provided time. If the string is in the
/// `MM-DD HH:MM` format, it will use the current year combined with the provided month, day, and time.
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
///
/// let month_day_time_str = String::from("07-27 15:30");
/// let utc_month_day_time = get_date_from_string(tz, &month_day_time_str);
/// println!("{}", utc_month_day_time); // Outputs the current year with the provided month, day, and time in UTC
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
    } else if let Ok(parsed_time) = NaiveDateTime::parse_from_str(date, "%Y-%m-%d %H:%M") {
        let event_date_with_timezone = tz
            .from_local_datetime(&parsed_time)
            .unwrap()
            .naive_utc()
            .and_utc();
        return event_date_with_timezone;
    } else {
        let parsed_time = NaiveDateTime::parse_from_str(
            &format!("{}-{}", Utc::now().year(), date),
            "%Y-%m-%d %H:%M",
        );
        print!("{}", &format!("{}-{}", Utc::now().year(), date));
        let combined_naive = parsed_time.unwrap();
        let event_date_with_timezone = tz
            .from_local_datetime(&combined_naive)
            .unwrap()
            .naive_utc()
            .and_utc();
        return event_date_with_timezone;
    }
}

#[cfg(test)]
mod tests {
    use chrono::{TimeZone, Weekday};

    use super::*;

    #[test]
    fn test_extracting_full_date_from_string() -> Result<(), String> {
        let tz: Tz = "America/New_York".parse().unwrap(); // UTC is 4 hours ahead of New York
        let date = String::from("2024-07-27 15:30");

        let actual_date = get_date_from_string(tz, &date);
        let expected_date = Utc.with_ymd_and_hms(2024, 07, 27, 19, 30, 0).unwrap();

        assert_eq!(actual_date, expected_date);
        Ok(())
    }

    #[test]
    fn test_extracting_full_date_except_year_from_string() -> Result<(), String> {
        let tz: Tz = "Europe/Zurich".parse().unwrap(); // UTC is 2 hours behind Zurich (CEST)
        let date = String::from("06-11 0:30");
        let current_year = Utc::now().year();

        let actual_date = get_date_from_string(tz, &date);
        let expected_date = Utc
            .with_ymd_and_hms(current_year, 06, 10, 22, 30, 0)
            .unwrap();

        assert_eq!(actual_date, expected_date);
        Ok(())
    }

    #[test]
    fn test_extracting_only_hours_and_minutes_from_string() -> Result<(), String> {
        let tz: Tz = "Asia/Tokyo".parse().unwrap(); // UTC is 9 hours behind Tokyo
        let date = String::from("23:12");
        let now = Utc::now();

        let actual_date = get_date_from_string(tz, &date);
        let expected_date = Utc
            .with_ymd_and_hms(now.year(), now.month(), now.day(), 14, 12, 0)
            .unwrap();

        assert_eq!(actual_date, expected_date);
        Ok(())
    }

    #[test]
    fn test_get_start_of_the_week() -> Result<(), String> {
        let now = Local::now();

        let start_of_the_week = get_start_of_the_week();

        let days_difference = now.signed_duration_since(start_of_the_week).num_days();        
        assert!(start_of_the_week <= now);
        assert!(days_difference >= 0 && days_difference <= 6);
        assert_eq!(start_of_the_week.weekday(), Weekday::Mon);
        Ok(())
    }
}

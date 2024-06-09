use google_calendar3::chrono::{DateTime, Datelike, Duration, Local};

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
        "Sunday"
    ];
    
    DAYS.iter().map(|&day| day.to_string()).collect()
}

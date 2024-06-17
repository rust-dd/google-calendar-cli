mod util;

use std::collections::HashMap;
use std::{collections::hash_map::Entry, fmt::Write};

use chrono_tz::Tz;
use clap::{Arg, Command};
use comfy_table::{Attribute, Cell, Color, ContentArrangement, Table};
use google_calendar3::chrono::{TimeZone, Timelike};
use google_calendar3::{
    api::{Event, EventDateTime},
    chrono::{Datelike, Duration, Month, NaiveDateTime, NaiveTime, Utc},
};
use util::calendar::{self, get_default_timezone};
use util::date::{days_in_english, get_start_of_the_week};

#[tokio::main]

async fn main() {
    let command = Command::new("gcal");
    let matches = command
        .about("Google Calendar - CLI")
        .version("0.0.1")
        .args_conflicts_with_subcommands(true)
        .arg(
            Arg::new("title")
                .help("Sets the event title")
                .required(false),
        )
        .arg(Arg::new("date").help("Sets the event date").required(false))
        .subcommand(
            Command::new("add")
                .about("Adds a new event to Google Calendar")
                .arg(
                    Arg::new("title")
                        .help("Sets the event title")
                        .required(true),
                )
                .arg(Arg::new("date").help("Sets the event date").required(true)),
        )
        .subcommand(Command::new("list").about("Lists all events in Google Calendar"))
        .get_matches();

    let hub = match calendar::auth().await {
        Ok(hub) => hub,
        Err(e) => {
            eprintln!("Error during authentication - {}", e);
            return;
        }
    };

    let tz: Tz = get_default_timezone(&hub).await.unwrap();

    match matches.subcommand() {
        Some(("list", _)) => {
            let start_of_the_week = get_start_of_the_week();
            let start_of_the_week_utc = start_of_the_week.to_utc();

            let events = hub
                .events()
                .list("primary")
                .time_min(start_of_the_week_utc)
                .time_max(start_of_the_week_utc + Duration::days(7))
                .doit()
                .await;
            match events {
                Ok((_, events)) => {
                    let mut table: Table = Table::new();
                    let mut event_dates: HashMap<_, Vec<_>> = HashMap::new();

                    if let Some(items) = events.items {
                        for event in items {
                            if event.start.clone().is_none()
                                || event.start.clone().unwrap().date_time.is_none()
                            {
                                // TODO: show full day event
                                continue;
                            }
                            let event_start =
                                event.clone().start.unwrap().date_time.unwrap().date_naive();
                            match event_dates.entry(event_start) {
                                Entry::Vacant(e) => {
                                    e.insert(vec![event]);
                                }
                                Entry::Occupied(mut e) => {
                                    e.get_mut().push(event);
                                }
                            }
                        }
                    }

                    let mut row_before_12: Vec<String> = vec![];
                    let mut row_after_12: Vec<String> = vec![];
                    let mut header: Vec<Cell> = vec![];
                    for (i, v) in days_in_english().iter().enumerate() {
                        let i = i as i64;
                        let next_date = start_of_the_week + Duration::days(i);
                        let header_value = format!(
                            "{} - {} {:?}",
                            v,
                            next_date.day(),
                            Month::try_from(u8::try_from(next_date.month()).unwrap())
                                .ok()
                                .unwrap()
                        );
                        if i < 5 {
                            header.push(
                                Cell::new(header_value)
                                    .fg(Color::DarkGreen)
                                    .add_attribute(Attribute::Bold),
                            );
                        } else {
                            header.push(Cell::new(header_value).fg(Color::DarkBlue));
                        }

                        let mut row_value_before_12: String = "".to_string();
                        let mut row_value_after_12: String = "".to_string();
                        if let Some(next_events) = event_dates.get(&next_date.date_naive()) {
                            let mut next_events_sorted: Vec<Event> = next_events.clone();
                            next_events_sorted.sort_by(|a, b| {
                                    a.start.as_ref()
                                        .unwrap()
                                        .date_time
                                        .unwrap()
                                        .cmp(&b.start.as_ref().unwrap().date_time.unwrap())
                                });
                            for next_event in next_events_sorted {
                                let event_start = next_event.start.unwrap().date_time.unwrap();
                                let summary = next_event.summary.unwrap().to_string();
                                let formatted_event = format!(
                                    "{:02}:{:02} - {:?}\n\n",
                                    tz.from_utc_datetime(&event_start.naive_local()).hour(),
                                    event_start.minute(),
                                    summary
                                );

                                if event_start.hour() < 12 {
                                    write!(row_value_before_12, "{}", formatted_event).unwrap();
                                } else {
                                    write!(row_value_after_12, "{}", formatted_event).unwrap();
                                }
                            }
                        }
                        row_before_12.push(row_value_before_12);
                        row_after_12.push(row_value_after_12);
                    }

                    table
                        .set_header(header)
                        .add_row(row_before_12)
                        .add_row(row_after_12)
                        .set_content_arrangement(ContentArrangement::DynamicFullWidth);

                    println!("{table}");
                }
                Err(e) => println!("Error retrieving events: {:?}", e),
            }
        }
        Some(("add", _)) | _ => {
            let title = matches.get_one::<String>("title");
            let date = matches.get_one::<String>("date");
            if title.is_none() {
                return;
            }

            if date.is_none() {
                let result = hub
                    .events()
                    .quick_add("primary", title.unwrap())
                    .doit()
                    .await;

                match result {
                    Ok((_, event)) => {
                        println!("Event created: {:?}", event.html_link.unwrap().to_string())
                    }
                    Err(e) => {
                        eprintln!("Error creating event: {:?}", e);
                    }
                }
            } else {
                let current_date = Utc::now().naive_utc();
                let parsed_time = NaiveTime::parse_from_str(date.unwrap(), "%H:%M");
                let combined = NaiveDateTime::new(current_date.date(), parsed_time.unwrap());
                let event = Event {
                    summary: Some(title.unwrap().clone()),
                    start: Some(EventDateTime {
                        date_time: Some(combined.and_utc()),
                        ..Default::default()
                    }),
                    end: Some(EventDateTime {
                        date_time: Some(combined.and_utc() + Duration::hours(1)),
                        ..Default::default()
                    }),
                    ..Default::default()
                };
                let result = hub.events().insert(event, "primary").doit().await;

                match result {
                    Ok((_, event)) => {
                        println!("Event created: {:?}", event.html_link.unwrap().to_string())
                    }
                    Err(e) => {
                        eprintln!("Error creating event: {:?}", e);
                    }
                }
            }
        }
    }
}

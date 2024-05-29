mod util;

use std::collections::HashMap;
use std::{collections::hash_map::Entry, fmt::Write};

use clap::{Arg, Command};
use comfy_table::{Attribute, Cell, Color, Table};
use google_calendar3::chrono::Timelike;
use google_calendar3::{
    api::{Event, EventDateTime},
    chrono::{Datelike, Duration, Local, Month, NaiveDateTime, NaiveTime, Utc},
};
use util::calendar;

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
        .subcommand(Command::new("add").about("Adds a new event to Google Calendar"))
        .subcommand(Command::new("list").about("Lists all events in Google Calendar"))
        .get_matches();

    let hub = match calendar::auth().await {
        Ok(hub) => hub,
        Err(e) => {
            eprintln!("Error during authentication: {}", e);
            return;
        }
    };

    match matches.subcommand() {
        Some(("list", _)) => {
            let events = hub
                .events()
                .list("primary")
                .time_min(Utc::now())
                .time_max(Utc::now() + Duration::days(7))
                .doit()
                .await;
            match events {
                Ok((_, events)) => {
                    let mut table = Table::new();
                    let now = Local::now();
                    let days_to_subtract = now.weekday().num_days_from_monday() as i64;
                    let start_of_the_week = now - Duration::days(days_to_subtract);

                    let mut event_dates: HashMap<_, _> = HashMap::new();

                    if let Some(items) = events.items {
                        for event in items {
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

                    let mut row: Vec<String> = vec![];
                    for i in 0..7 {
                        let next_date = start_of_the_week + Duration::days(i);
                        let mut value = format!(
                            "{} {:?}",
                            next_date.day(),
                            Month::try_from(u8::try_from(next_date.month()).unwrap())
                                .ok()
                                .unwrap()
                        );
                        write!(value, "\n\n").unwrap();
                        
                        let next_events = event_dates.get(&next_date.date_naive());
                        if next_events.is_some() {
                            let next_events_detail: Vec<Event> = next_events.unwrap().clone();
                            for next_event_details in next_events_detail {
                                let next_event_details_start =
                                    next_event_details.start.unwrap().date_time.unwrap();
                                write!(value, "\n\n").unwrap();
                                write!(
                                    value,
                                    "{:?}:{:?} {:?}",
                                    next_event_details_start.hour(),
                                    next_event_details_start.minute(),
                                    next_event_details.summary.unwrap().to_string()
                                )
                                .unwrap();
                            }
                        }
                        row.push(value);
                    }

                    table
                        .set_header(vec![
                            Cell::new("Monday")
                                .fg(Color::Green)
                                .add_attribute(Attribute::Bold),
                            Cell::new("Tuesday")
                                .fg(Color::Green)
                                .add_attribute(Attribute::Bold),
                            Cell::new("Wednesday")
                                .fg(Color::Green)
                                .add_attribute(Attribute::Bold),
                            Cell::new("Thursday")
                                .fg(Color::Green)
                                .add_attribute(Attribute::Bold),
                            Cell::new("Friday")
                                .fg(Color::Green)
                                .add_attribute(Attribute::Bold),
                            Cell::new("Saturday").fg(Color::Blue),
                            Cell::new("Sunday").fg(Color::Blue),
                        ])
                        .add_row(row);

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
                    Ok((_, event)) => println!("Event created: {:?}", event),
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
                    Ok((_, event)) => println!("Event created: {:?}", event),
                    Err(e) => {
                        eprintln!("Error creating event: {:?}", e);
                    }
                }
            }
        }
    }
}

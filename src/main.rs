mod util;

use std::collections::HashMap;
use std::{collections::hash_map::Entry, fmt::Write};

use clap::{Arg, Command};
use comfy_table::{Attribute, Cell, Color, ContentArrangement, Table};
use google_calendar3::chrono::Timelike;
use google_calendar3::{
    api::{Event, EventDateTime},
    chrono::{Datelike, Duration, Month, NaiveDateTime, NaiveTime, Utc},
};
use util::calendar;
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
                        println!("{}", i);
                        if i < 5 {
                            header.push(
                                Cell::new(header_value)
                                    .fg(Color::DarkGreen)
                                    .add_attribute(Attribute::Bold),
                            );
                        } else {
                            header.push(
                                Cell::new(header_value)
                                    .fg(Color::DarkBlue)
                            );
                        }
                        

                        let mut row_value: String = "".to_string();
                        if let Some(next_events) = event_dates.get(&next_date.date_naive()) {
                            let next_events_detail: Vec<Event> = next_events.clone();
                            for next_event_details in next_events_detail {
                                let next_event_details_start =
                                    next_event_details.start.unwrap().date_time.unwrap();
                                write!(
                                    row_value,
                                    "{:02}:{:02} {:?}",
                                    next_event_details_start.hour(),
                                    next_event_details_start.minute(),
                                    next_event_details.summary.unwrap().to_string()
                                )
                                .unwrap();
                                write!(row_value, "\n\n").unwrap();
                            }
                        }
                        row.push(row_value);
                    }

                    table
                        .set_header(header)
                        .add_row(row)
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

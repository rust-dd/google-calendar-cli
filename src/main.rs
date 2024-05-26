mod util;

use clap::{Arg, Command};
use google_calendar3::{
    api::{Event, EventDateTime}, chrono::{Duration, NaiveDateTime, NaiveTime, Utc}
};
use util::auth;

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

    let hub = auth::auth().await.expect("Failed to authenticate and create CalendarHub");

    match matches.subcommand() {
        Some(("list", _)) => {
            let events = hub
                .events()
                .list("primary")
                .time_min(Utc::now())
                .doit()
                .await;
            match events {
                Ok((_, events)) => {
                    if let Some(items) = events.items {
                        for event in items {
                            println!(
                                "{:?}, {:?}-{:?}, {:?}",
                                event.summary, event.start, event.end, event.html_link
                            );
                        }
                    }
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

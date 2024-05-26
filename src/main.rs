mod util;

use clap::{Arg, Command};
use google_calendar3::{
    api::{Event, EventDateTime},
    chrono::{Duration, NaiveDateTime, NaiveTime, Utc},
    hyper, hyper_rustls,
    oauth2::{self, ApplicationSecret},
    CalendarHub,
};

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
        .subcommand(
            Command::new("auth")
                .about("User authentication")
                .subcommand(Command::new("login").about("Login to the application"))
                .subcommand(Command::new("logout").about("Logout from the application")),
        )
        .get_matches();

    let secret: oauth2::ApplicationSecret = ApplicationSecret {
        auth_uri: "https://accounts.google.com/o/oauth2/auth".to_string(),
        client_secret: "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".to_string(),
        token_uri: "https://accounts.google.com/o/oauth2/token".to_string(),
        redirect_uris: vec!["urn:ietf:wg:oauth:2.0:oob".to_string()],
        client_id: "602236549045-3gcv7m50sp1d6vvqklimb5oaasp9ihi9.apps.googleusercontent.com"
            .to_string(),
        auth_provider_x509_cert_url: Some("https://www.googleapis.com/oauth2/v1/certs".to_string()),
        project_id: None,
        client_email: None,
        client_x509_cert_url: None,
    };

    let secret_path = util::file::get_absolute_path(".gcal/secret.json").unwrap();
    let _ = util::file::ensure_directory_exists(&secret_path);
    let auth = oauth2::InstalledFlowAuthenticator::builder(
        secret,
        oauth2::InstalledFlowReturnMethod::Interactive,
    )
    .persist_tokens_to_disk(&secret_path.to_str().unwrap())
    .build()
    .await
    .unwrap();

    let scopes = &[
        "https://www.googleapis.com/auth/calendar",
        "https://www.googleapis.com/auth/calendar.events",
        "https://www.googleapis.com/auth/calendar.readonly",
        "https://www.googleapis.com/auth/calendar.events.readonly",
    ];

    match auth.token(scopes).await {
        Ok(_) => println!("User is authenticated."),
        Err(e) => println!("error: {:?}", e),
    }

    let hub = CalendarHub::new(
        hyper::Client::builder().build(
            hyper_rustls::HttpsConnectorBuilder::new()
                .with_native_roots()
                .https_or_http()
                .enable_http2()
                .build(),
        ),
        auth,
    );

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

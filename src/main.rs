use clap::{Arg, Command};
use google_calendar3::{
    chrono::{Utc}, hyper, hyper_rustls, oauth2::{self, ApplicationSecret}, CalendarHub
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
        .subcommand(Command::new("add"))
        .subcommand(Command::new("list").about("list .."))
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

    let auth = oauth2::InstalledFlowAuthenticator::builder(
        secret,
        oauth2::InstalledFlowReturnMethod::Interactive,
    )
    .persist_tokens_to_disk("tokencache.json")
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
            let events = hub.events().list("primary").time_min(Utc::now()).doit().await;
            match events {
                Ok((_, events)) => {
                    if let Some(items) = events.items {
                        for event in items {
                            println!("{:?}, {:?}-{:?}, {:?}", event.summary, event.start, event.end, event.html_link);
                        }
                    }
                }
                Err(e) => println!("Error retrieving events: {:?}", e),
            }
        },
        Some(("add", _)) | _ => {
            let title = matches.get_one::<String>("title");
            if title.is_none() {
                return;
            }
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
        },
    }
}

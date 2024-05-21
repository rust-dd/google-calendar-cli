use clap::{arg, Arg, Command};
use google_calendar3::{api::Channel, chrono, hyper, hyper_rustls, oauth2::{self, ApplicationSecret}, CalendarHub, Error};

#[tokio::main]

async fn main() {
    let matches = Command::new("gcal")
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

    println!("-title used: {:?}", matches.get_one::<String>("title"));
    println!("-date used: {:?}", matches.get_one::<String>("date"));

    // Get an ApplicationSecret instance by some means. It contains the `client_id` and
    // `client_secret`, among other things.
    let secret: oauth2::ApplicationSecret = ApplicationSecret {
        auth_uri: "https://accounts.google.com/o/oauth2/auth".to_string(),
        client_secret: "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".to_string(),
        token_uri: "https://accounts.google.com/o/oauth2/token".to_string(),
        redirect_uris: vec!["urn:ietf:wg:oauth:2.0:oob".to_string()],
        client_id: "602236549045-3gcv7m50sp1d6vvqklimb5oaasp9ihi9.apps.googleusercontent.com".to_string(),
        auth_provider_x509_cert_url: Some("https://www.googleapis.com/oauth2/v1/certs".to_string()),
        project_id: None,
        client_email: None,
        client_x509_cert_url: None,
        
    };
    // Instantiate the authenticator. It will choose a suitable authentication flow for you,
    // unless you replace  `None` with the desired Flow.
    // Provide your own `AuthenticatorDelegate` to adjust the way it operates and get feedback about
    // what's going on. You probably want to bring in your own `TokenStorage` to persist tokens and
    // retrieve them from storage.
    println!("------------------------------");

    let auth = oauth2::InstalledFlowAuthenticator::builder(
        secret,
        oauth2::InstalledFlowReturnMethod::Interactive,
    )
    .persist_tokens_to_disk("tokencache.json")
    .build()
    .await
    .unwrap();

    let scopes = &[
        "https://www.googleapis.com/auth/calendar.readonly", 
        "https://www.googleapis.com/auth/calendar.events.readonly"
    ];

    match auth.token(scopes).await {
        Ok(_) => println!("User is authenticated."),
        Err(e) => println!("error: {:?}", e),
    }

    println!("-------------------------------------------");

    let hub = CalendarHub::new(
        hyper::Client::builder().build(
            hyper_rustls::HttpsConnectorBuilder::new()
                .with_native_roots()
                .https_or_http()
                .enable_http1()
                .build(),
        ),
        auth,
        
    ); 

    // As the method needs a request, you would usually fill it with the desired information
    // into the respective structure. Some of the parts shown here might not be applicable !
    // Values shown here are possibly random and not representative !
    let calendar_list = hub.calendar_list().list().doit().await;
    match calendar_list {
        Ok((_, calendar_list)) => {
            println!("Calendar List:");
            if let Some(items) = calendar_list.items {
                for calendar in items {
                    println!("{:?}", calendar);
                }
            }
        }
        Err(e) => println!("Error retrieving calendar list: {:?}", e),
    }

    println!("-------------------------------------------");

    // List calendars
    let calendar_list = hub.calendar_list().list().doit().await;
    match calendar_list {
        Ok((_, calendar_list)) => {
            println!("Calendar List:");
            if let Some(items) = calendar_list.items {
                for calendar in items {
                    println!("{:?}", calendar);
                }
            }
        }
        Err(e) => println!("Error retrieving calendar list: {:?}", e),
    }

    println!("-------------------------------------------");

    // Print events from the primary calendar
    let events = hub.events().list("primary").doit().await;
    match events {
        Ok((_, events)) => {
            println!("Events:");
            if let Some(items) = events.items {
                for event in items {
                    println!("{:?}", event);
                }
            }
        }
        Err(e) => println!("Error retrieving events: {:?}", e),
    }

   
}

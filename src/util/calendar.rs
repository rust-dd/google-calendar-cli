use std::{error::Error, path::Path};

use google_calendar3::{
    hyper::{self, client::HttpConnector},
    hyper_rustls::{self, HttpsConnector},
    oauth2::{self, ApplicationSecret},
    CalendarHub,
};

use super::file;

/// Authenticates the user with Google Calendar API and returns a CalendarHub instance.
///
/// ## Returns
///
/// * `Result<CalendarHub<HttpsConnector<HttpConnector>>, Box<dyn Error>>` - A result containing the CalendarHub instance or an error if any step fails.
///
/// ## Errors
///
/// This function will return an error if:
/// - The home directory cannot be determined.
/// - The secret JSON file cannot be read.
/// - The authenticator fails to build or retrieve tokens.
/// - Any other I/O or network errors occur during these operations.
pub async fn auth() -> Result<CalendarHub<HttpsConnector<HttpConnector>>, Box<dyn Error>> {
    let secret_absolute_path = file::get_absolute_path(".gcal/secret.json")?;
    let secret_path = std::path::Path::new(&secret_absolute_path);
    let _ = file::ensure_directory_exists(secret_path);
    let secret = read_google_secret(secret_path).await?;

    let store_path = file::get_absolute_path(".gcal/store.json")?;
    let auth = oauth2::InstalledFlowAuthenticator::builder(
        secret,
        oauth2::InstalledFlowReturnMethod::Interactive,
    )
    .persist_tokens_to_disk(&store_path.to_str().unwrap())
    .build()
    .await?;

    let scopes = &[
        "https://www.googleapis.com/auth/calendar",
        "https://www.googleapis.com/auth/calendar.events",
        "https://www.googleapis.com/auth/calendar.readonly",
        "https://www.googleapis.com/auth/calendar.events.readonly",
    ];

    match auth.token(scopes).await {
        Ok(_) => println!("User is authenticated."),
        Err(e) => println!("Authentication error: {:?}", e),
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
    Ok(hub)
}

/// Reads the Google application secret from the specified path.
///
/// This function reads and parses the Google application secret JSON file into an ApplicationSecret structure.
///
/// ## Arguments
///
/// * `path` - A reference to the Path of the Google application secret JSON file.
///
/// ## Returns
///
/// * `Result<ApplicationSecret, Box<dyn Error>>` - A result containing the ApplicationSecret or an error if the file cannot be read.
///
/// ## Errors
///
/// This function will return an error if:
/// - The file cannot be read.
/// - The contents of the file cannot be parsed into an ApplicationSecret.
async fn read_google_secret(path: &Path) -> Result<ApplicationSecret, Box<dyn Error>> {
    Ok(oauth2::read_application_secret(path).await?)
}
use std::{error::Error, path::Path};

use google_calendar3::{
    hyper::{self, client::HttpConnector},
    hyper_rustls::{self, HttpsConnector},
    oauth2::{self, ApplicationSecret},
    CalendarHub,
};

use super::file;

pub async fn auth() -> Result<CalendarHub<HttpsConnector<HttpConnector>>, Box<dyn Error>> {
    let secret_absolute_path = file::get_absolute_path(".gcal/secret.json").unwrap();
    let secret_path = std::path::Path::new(&secret_absolute_path);
    let _ = file::ensure_directory_exists(secret_path);
    let secret = read_google_secret(secret_path).await?;

    let store_path = file::get_absolute_path(".gcal/store.json").unwrap();
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
    Ok(hub)
}

async fn read_google_secret(path: &Path) -> Result<ApplicationSecret, Box<dyn Error>> {
    Ok(oauth2::read_application_secret(path).await?)
}

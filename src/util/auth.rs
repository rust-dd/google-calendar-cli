use std::{io::Error, path::Path};

use google_calendar3::oauth2::{self, ApplicationSecret};

pub async fn read_google_secret(path: &Path) -> Result<ApplicationSecret, Error>  {
    return oauth2::read_application_secret(path).await;
}
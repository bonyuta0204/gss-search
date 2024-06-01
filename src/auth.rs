use std::{fs, io::Error};

use google_sheets4::{
    hyper::client::HttpConnector,
    oauth2::{
        authenticator::Authenticator, hyper_rustls::HttpsConnector, read_application_secret,
        ApplicationSecret, InstalledFlowAuthenticator, InstalledFlowReturnMethod,
    },
};

use crate::path::path_for_tokencache;

pub async fn build_secret_from_json() -> Result<ApplicationSecret, Error> {
    read_application_secret("clientsecret.json").await
}

pub async fn build_secret_from_env() -> Result<ApplicationSecret, String> {
    let client_id = std::env::var("GOOGLE_CLIENT_ID")
        .map_err(|_| "GOOGLE_CLIENT_ID not set in environment".to_string())?;
    let client_secret = std::env::var("GOOGLE_CLIENT_SECRET")
        .map_err(|_| "GOOGLE_CLIENT_SECRET not set in environment".to_string())?;
    let project_id = std::env::var("GOOGLE_PROJECT_ID")
        .map_err(|_| "GOOGLE_PROJECT_ID not set in environment".to_string())?;

    Ok(ApplicationSecret {
        client_id,
        client_secret,
        project_id: Some(project_id),
        token_uri: "https://oauth2.googleapis.com/token".to_string(),
        auth_uri: "https://accounts.google.com/o/oauth2/auth".to_string(),
        redirect_uris: vec!["http://localhost".to_string()],
        auth_provider_x509_cert_url: Some("https://www.googleapis.com/oauth2/v1/certs".to_string()),
        client_email: None,
        client_x509_cert_url: None,
    })
}

// Create an authenticator that uses an InstalledFlow to authenticate. The
// authentication tokens are persisted to a file named tokencache.json. The
// authenticator takes care of caching tokens to disk and refreshing tokens once
// they've expired.
pub async fn build_auth(
    secret: ApplicationSecret,
) -> Result<Authenticator<HttpsConnector<HttpConnector>>, String> {
    let tokencache_path = path_for_tokencache();
    if let Some(parent) = tokencache_path.parent() {
        if !parent.exists() {
            fs::create_dir_all(parent).expect("Failed to create directories");
        }
    }
    InstalledFlowAuthenticator::builder(secret, InstalledFlowReturnMethod::HTTPRedirect)
        .persist_tokens_to_disk(tokencache_path)
        .build()
        .await
        .map_err(|e| format!("Failed to authenticate. {}", e))
}

// build auth using json
pub async fn create_auth() -> Result<Authenticator<HttpsConnector<HttpConnector>>, String> {
    let secret = match build_secret_from_env().await {
        Ok(secret) => secret,
        Err(_env_err) => build_secret_from_json().await.map_err(|_json_err| {
            format!("Failed to read secret from Environment Variable of JSON file")
        })?,
    };
    build_auth(secret).await
}

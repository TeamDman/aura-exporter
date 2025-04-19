use crate::remote_types::auth_response::AuthResponse;
use eyre::Context;
use eyre::bail;
use reqwest::Client;
use serde_json::json;
use tracing::debug;
use std::path::Path;
use tokio::sync::OnceCell;
use tracing::info;
use tracing::warn;

const AUTH_FILE_NAME: &str = "aura-auth.json";

pub async fn login() -> eyre::Result<()> {
    let email = std::env::var("AURA_EMAIL");
    let password = std::env::var("AURA_PASSWORD");
    let (email, password) = match (email, password) {
        (Ok(email), Ok(password)) => (email, password),
        (Err(_), Err(_)) => {
            bail!("Please set the AURA_EMAIL and AURA_PASSWORD environment variables.");
        }
        (Ok(_email), Err(_)) => {
            bail!("Please set the AURA_PASSWORD environment variable.");
        }
        (Err(_), Ok(_password)) => {
            bail!("Please set the AURA_EMAIL environment variable.");
        }
    };

    info!("Login command executing, environment variables were properly set :D");
    let login_url = "https://api.pushd.com/v5/login.json";
    let login_payload = json!({
        "identifier_for_vendor": "does-not-matter",
        "client_device_id": "does-not-matter",
        "app_identifier": "com.pushd.Framelord",
        "locale": "en",
        "user": {
            "email": email,
            "password": password
        }
    });
    let result = reqwest::Client::new()
        .post(login_url)
        .header("Content-Type", "application/json")
        .json(&login_payload)
        .send()
        .await?;
    if result.status().is_success() {
        let response_text = result.text().await?;
        info!("Login successful!");

        info!("Writing auth details to {AUTH_FILE_NAME}");
        warn!("Remove this credential by running the logout command when you are done!");
        let response_json: serde_json::Value = serde_json::from_str(&response_text)?;
        let response_json_pretty = serde_json::to_string_pretty(&response_json)?;
        tokio::fs::write(AUTH_FILE_NAME, response_json_pretty).await?;
    } else {
        let error_text = result.text().await?;
        bail!("Login failed: {}", error_text);
    }
    Ok(())
}

pub async fn logout() -> eyre::Result<()> {
    let path = Path::new(AUTH_FILE_NAME);
    if path.exists() {
        tokio::fs::remove_file(path).await?;
        info!("Logout successful, auth file removed.");
    } else {
        info!("No auth file found, already logged out.");
    }
    Ok(())
}

pub async fn load_auth_data() -> eyre::Result<AuthResponse> {
    let path = Path::new(AUTH_FILE_NAME);
    if !path.exists() {
        bail!("Not logged in. Please run the login command first.");
    }

    let auth_file_content = tokio::fs::read_to_string(path).await?;
    let auth_data: AuthResponse = serde_json::from_str(&auth_file_content)?;
    Ok(auth_data)
}

const CLIENT: OnceCell<Client> = OnceCell::const_new();

pub async fn get_authenticated_client() -> eyre::Result<Client> {
    if let Some(client) = CLIENT.get() {
        debug!("Using existing authenticated client");
        Ok(client.clone())
    } else {
        debug!("Creating new authenticated client");
        let client = create_authenticated_client().await?;
        CLIENT.set(client.clone()).wrap_err(eyre::eyre!(
            "Failed to set authenticated client in OnceCell"
        ))?;
        Ok(client)
    }
}

pub async fn create_authenticated_client() -> eyre::Result<Client> {
    let auth_data = load_auth_data().await?;
    let user_id = auth_data.result.current_user.id;
    let auth_token = auth_data.result.current_user.auth_token;

    let client = Client::builder()
        .default_headers({
            let mut headers = reqwest::header::HeaderMap::new();
            headers.insert(
                "X-User-Id",
                reqwest::header::HeaderValue::from_str(&user_id)?,
            );
            headers.insert(
                "X-Token-Auth",
                reqwest::header::HeaderValue::from_str(&auth_token)?,
            );
            headers
        })
        .build()?;

    debug!("Created authenticated client");
    Ok(client)
}

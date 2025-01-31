use chrono::{Datelike, NaiveDate, Utc};
use reqwest::{Client, StatusCode, header};
use serde_json::Value;
use std::error::Error;
use reqwest::header::HeaderMap;

const GITHUB_API_URL: &str = "https://api.github.com";

/// Fetch events from the user's general activity.
async fn fetch_user_events(
    client: &Client,
    username: &str,
    token: Option<&str>,
) -> Result<Vec<Value>, Box<dyn Error>> {
    let mut all_events = Vec::new();
    let mut page = 1;

    loop {
        let url = format!(
            "{}/users/{}/events?per_page=100&page={}",
            GITHUB_API_URL, username, page
        );
        let mut request = client.get(&url).header("User-Agent", "github-activity-cli");

        if let Some(t) = token {
            request = request.header("Authorization", format!("token {}", t));
        }

        let response = request.send().await?;
        if response.status() != StatusCode::OK {
            return Err(format!("Failed to fetch user events: {}", response.status()).into());
        }

        let headers = response.headers().clone();
        let events: Value = response.json().await?;

        let filtered_events = filter_events_for_last_year(&events)?;

        all_events.extend(filtered_events);

        if !has_next_page(headers) {
            break;
        }
        page += 1;
    }

    Ok(all_events)
}

/// Fetch the list of public repositories for a user.
async fn fetch_user_repos(
    client: &Client,
    username: &str,
    token: Option<&str>,
) -> Result<Vec<Value>, Box<dyn Error>> {
    let repos_url = format!("{}/users/{}/repos?per_page=100", GITHUB_API_URL, username);
    let mut request = client
        .get(&repos_url)
        .header("User-Agent", "github-activity-cli");

    if let Some(t) = token {
        request = request.header("Authorization", format!("token {}", t));
    }

    let response = request.send().await?;
    if response.status() != StatusCode::OK {
        return Err(format!("Failed to fetch user repositories: {}", response.status()).into());
    }

    let repos: Value = response.json().await?;
    Ok(repos.as_array().unwrap_or(&vec![]).to_vec())
}

/// Fetch events from a specific repository.
async fn fetch_repo_events(
    client: &Client,
    username: &str,
    repo_name: &str,
    token: Option<&str>,
) -> Result<Vec<Value>, Box<dyn Error>> {
    let repo_events_url = format!(
        "{}/repos/{}/{}/events?per_page=100",
        GITHUB_API_URL, username, repo_name
    );
    let mut request = client
        .get(&repo_events_url)
        .header("User-Agent", "github-activity-cli");

    if let Some(t) = token {
        request = request.header("Authorization", format!("token {}", t));
    }

    let response = request.send().await?;
    if response.status() != StatusCode::OK {
        return Err(format!("Failed to fetch repository events: {}", response.status()).into());
    }

    let repo_events: Value = response.json().await?;
    Ok(repo_events.as_array().unwrap_or(&vec![]).to_vec())
}

/// Filter events to keep only those that happened in the last year.
fn filter_events_for_last_year(events: &Value) -> Result<Vec<Value>, Box<dyn Error>> {
    let current_year = Utc::now().naive_utc().year();
    let filtered_events = events
        .as_array()
        .unwrap_or(&vec![])
        .iter()
        .filter(|event| {
            if let Some(created_at) = event.get("created_at").and_then(|v| v.as_str()) {
                if let Ok(date) = NaiveDate::parse_from_str(&created_at[..10], "%Y-%m-%d") {
                    return date.year() == current_year;
                }
            }
            false
        })
        .cloned()
        .collect();
    Ok(filtered_events)
}

/// Check if there are more pages based on the 'Link' header.
fn has_next_page(headers: HeaderMap) -> bool {
    if let Some(link_header) = headers.get(header::LINK) {
        let link_header_str = link_header.to_str().unwrap_or_default();
        return link_header_str.contains("rel=\"next\"");
    }
    false
}

/// Main function that fetches and combines both user and repository events.
pub(crate) async fn fetch_github_activity(
    username: &str,
    token: Option<&str>
) -> Result<Value, Box<dyn Error>> {
    let client = Client::new();

    let user_events = fetch_user_events(&client, username, token).await?;

    let repos = fetch_user_repos(&client, username, token).await?;

    let mut all_events = user_events;

    for repo in repos.iter() {
        if let Some(repo_name) = repo.get("name").and_then(|v| v.as_str()) {
            let repo_events = fetch_repo_events(&client, username, repo_name, token).await?;
            all_events.extend(repo_events);
        }
    }

    Ok(Value::Array(all_events))
}

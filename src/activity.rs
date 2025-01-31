use chrono::{Duration, Utc};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct ContributionDay {
    pub date: String,
    #[serde(rename = "contributionCount")]
    pub contribution_count: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContributionWeek {
    #[serde(rename = "contributionDays")]
    pub contribution_days: Vec<ContributionDay>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContributionCalendar {
    pub weeks: Vec<ContributionWeek>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContributionsCollection {
    #[serde(rename = "contributionCalendar")]
    pub contribution_calendar: ContributionCalendar,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    #[serde(rename = "contributionsCollection")]
    pub contributions_collection: ContributionsCollection,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GraphQLResponse {
    pub data: Option<HashMap<String, User>>,
    pub errors: Option<Vec<Value>>,
}

pub async fn fetch_last_year_contributions(
    username: &str,
    github_pat: &str,
) -> Result<Vec<u32>, Box<dyn std::error::Error>> {
    let client = Client::new();
    let url = "https://api.github.com/graphql";

    let now = Utc::now();
    let last_year = now - Duration::days(190);

    let query = format!(
        r#"
        query {{
            user(login: "{username}") {{
                contributionsCollection(from: "{from}", to: "{to}") {{
                    contributionCalendar {{
                        weeks {{
                            contributionDays {{
                                date
                                contributionCount
                            }}
                        }}
                    }}
                }}
            }}
        }}
        "#,
        username = username,
        from = last_year.to_rfc3339(),
        to = now.to_rfc3339()
    );

    let response = client
        .post(url)
        .header("Authorization", format!("Bearer {}", github_pat))
        .header("Content-Type", "application/json")
        .header("User-Agent", "Github Stats")
        .body(serde_json::to_string(
            &serde_json::json!({ "query": query }),
        )?)
        .send()
        .await?;

    let response_text = response.text().await?;
    let graphql_response: GraphQLResponse = serde_json::from_str(&response_text)?;

    if let Some(errors) = graphql_response.errors {
        return Err(format!("GraphQL Errors: {:?}", errors).into());
    }

    if let Some(data) = graphql_response.data {
        if let Some(user) = data.get("user") {
            let mut contributions = Vec::new();
            for week in &user.contributions_collection.contribution_calendar.weeks {
                for day in &week.contribution_days {
                    contributions.push(day.contribution_count);
                }
            }
            return Ok(contributions);
        }
    }

    Err("Unexpected response format".into())
}

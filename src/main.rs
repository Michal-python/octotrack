mod activity;
mod cli;
mod display;
mod fetch;
mod spinner;
mod streak;

use crate::activity::fetch_last_year_contributions;
use crate::display::display_contributions_by_month;
use crate::spinner::spawn_spinner_task;
use crate::streak::fetch_streak_data;
use cli::get_matches;
use colored::*;
use display::display_activity;
use fetch::fetch_github_activity;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::{env};
use tokio::sync::Mutex;

#[tokio::main]
async fn main() {
    let matches = get_matches();
    let username = matches.get_one::<String>("username").unwrap();
    let detailed = matches.get_flag("detailed");
    let json_output = matches.get_flag("json");
    let show_streak = matches.get_flag("streak");
    let show_contributions = matches.get_flag("contributions");

    let github_token = env::var("GITHUB_TOKEN").ok();
    let running = Arc::new(Mutex::new(AtomicBool::new(true)));

    spawn_spinner_task(running.clone());

    match fetch_github_activity(username, github_token.as_deref()).await {
        Ok(activity) => {
            if json_output {
                println!("{}", serde_json::to_string_pretty(&activity).unwrap());
            } else {
                let streak = fetch_streak_data(username)
                    .await
                    .expect("Couldn't fetch streak data");
                let contributions = if github_token.is_some() && show_contributions {
                    fetch_last_year_contributions(username, github_token.unwrap().as_str())
                        .await
                        .expect("Could not fetch last year contributions")
                } else {
                    vec![]
                };

                running
                    .lock()
                    .await
                    .store(false, std::sync::atomic::Ordering::Relaxed);
                print!("\r");

                display_activity(username, &activity, streak, detailed, show_streak);

                if !contributions.is_empty() && show_contributions {
                    display_contributions_by_month(contributions);
                }
            }
        }
        Err(e) => eprintln!("{} {}", "Error:".red(), e),
    }
}

use std::cmp::min;
use chrono::{Datelike, NaiveDate, Utc};
use colored::*;
use serde_json::Value;
use std::collections::HashMap;

const MONTH_NAMES: [&str; 12] = [
    "January",
    "February",
    "March",
    "April",
    "May",
    "June",
    "July",
    "August",
    "September",
    "October",
    "November",
    "December",
];

pub fn display_activity(username: &str, activity: &Value, streak: (String, String, String), detailed: bool, show_streak: bool) {
    println!(
        "{} {}'s GitHub Latest Activity",
        "🔵".blue(),
        username.bold()
    );
    println!("{}", "────────────────────────────────".blue());
    let mut commit_count = 0;
    let mut pr_count = 0;
    let mut issue_count = 0;
    let mut star_count = 0;
    let mut streak_map: HashMap<u32, u32> = HashMap::new();

    if let Some(events) = activity.as_array() {
        for event in events.iter() {
            if let Some(event_type) = event.get("type").and_then(|v| v.as_str()) {
                if let Some(created_at) = event.get("created_at").and_then(|v| v.as_str()) {
                    if let Ok(date) = NaiveDate::parse_from_str(&created_at[..10], "%Y-%m-%d") {
                        let month = date.month();
                        *streak_map.entry(month).or_insert(0) += 1;
                    }
                }
                match event_type {
                    "PushEvent" => commit_count += 1,
                    "PullRequestEvent" => pr_count += 1,
                    "IssuesEvent" => issue_count += 1,
                    "WatchEvent" => star_count += 1,
                    _ => {}
                }
            }
        }
    }
    println!(
        "{} Commits: {}",
        "📌".cyan(),
        commit_count.to_string().bold()
    );
    println!(
        "{} Pull Requests: {}",
        "🔀".green(),
        pr_count.to_string().bold()
    );
    println!(
        "{} Issues: {}",
        "🐛".yellow(),
        issue_count.to_string().bold()
    );
    println!(
        "{} Stars: {}",
        "⭐".magenta(),
        star_count.to_string().bold()
    );
    println!("{}", "────────────────────────────────".blue());

    if show_streak {
        display_streak_data(streak.0, streak.1, streak.2);
    }
    if detailed {
        println!("\n{} Recent Activity:", "📝".blue());
        for event in activity.as_array().unwrap_or(&vec![]).iter().take(10) {
            if let Some(event_type) = event.get("type").and_then(|v| v.as_str()) {
                let repo_name = event
                    .get("repo")
                    .and_then(|r| r.get("name"))
                    .and_then(|n| n.as_str())
                    .unwrap_or("Unknown");
                println!(
                    " - {} {} in {}",
                    "⚡".purple(),
                    event_type.bold(),
                    repo_name.underline()
                );
            }
        }
    }
}

pub fn display_contributions_by_month(contributions: Vec<u32>) {
    let today = Utc::now().naive_utc();

    let mut contributions_per_month: HashMap<u32, u32> = HashMap::new();

    for (day_index, contribution_count) in contributions.iter().enumerate() {
        let day = today - chrono::Duration::days(day_index as i64);
        let month = day.month();

        *contributions_per_month.entry(month).or_insert(0) += *contribution_count;
    }

    println!("{} Contributions by Month:", "📊".blue());
    println!("{}", "────────────────────────────────".blue());

    let mut streak_length = 0;
    let mut max_streak = 0;

    for month in 1..=12 {
        let count = *contributions_per_month.get(&month).unwrap_or(&0);
        if count == 0 {
            continue;
        }
        let bar = "█".repeat(min(count, 20) as usize) + &"▓".repeat(10 - min(count, 10) as usize / 2);

        println!(
            "{}: {} {} contributions",
            MONTH_NAMES[(month - 1) as usize],
            bar.bright_green(),
            count
        );

        if count > 0 {
            streak_length += 1;
            if streak_length > max_streak {
                max_streak = streak_length;
            }
        } else {
            streak_length = 0;
        }
    }

    println!("\nLongest Streak: {} months", max_streak);
}

pub fn display_streak_data(total_contributions: String, current_streak: String, longest_streak: String) {
    println!("{}", "🔵 GitHub Streak Information".blue());
    println!("{}", "────────────────────────────────".blue());

    println!(
        "{} Total Contributions: {}",
        "📅".cyan(),
        total_contributions.trim().bold()
    );

    let current_streak_int: u32 = current_streak.trim().parse().unwrap_or(0);
    println!(
        "{} Current Streak: {} days",
        "🔥".red(),
        current_streak.trim().bold()
    );

    let longest_streak_int: u32 = longest_streak.trim().parse().unwrap_or(0);
    println!(
        "{} Longest Streak: {} days",
        "🏆".green(),
        longest_streak.trim().bold()
    );

    if current_streak_int == longest_streak_int {
        println!(
            "{} Keep up the great work! You're on a roll!",
            "💪".yellow()
        );
    } else {
        println!(
            "{} You are doing great! Let's aim to break your longest streak!",
            "🚀".yellow()
        );
    }

    println!("{}", "────────────────────────────────".blue());
}
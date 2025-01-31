# octotrack

A Rust-based tool to display and analyze GitHub user activity, including contributions, streaks, and detailed event logs. This project provides a clean and colorful CLI interface to visualize GitHub activity data.

## Features

- **Activity Summary**: Displays the total number of commits, pull requests, issues, and stars.
- **Streak Information**: Shows the current streak, longest streak, and total contributions.
- **Monthly Contributions**: Visualizes contributions by month with a bar chart and calculates the longest streak of active months.
- **Colorful CLI Output**: Uses colored text and emojis for a visually appealing experience.

## Usage

### Prerequisites

- Rust installed on your machine.
- A GitHub API token (optional, if you need to fetch contribution data).

### Installation

1. Clone the repository:
   ```bash
   git clone https://github.com/Michal-python/octotrack.git
   cd octotrack
   ```
2. Build the project:
   ```bash
   cargo build --release
   ```
3. Run the project:
   ```bash
   cargo run -- --username YOUR_GITHUB_USERNAME
   ```
4. If you want to use GitHub token, before running the binary run:
    ```bash
    export GITHUB_TOKEN=<token>
    ```
   
### Example output
```
🔵 JohnDoe's GitHub Latest Activity
────────────────────────────────
📌 Commits: 42
🔀 Pull Requests: 10
🐛 Issues: 5
⭐ Stars: 8

📊 Contributions by Month:
────────────────────────────────
January: █████▓▓▓▓▓ 15 contributions
February: ███████▓▓▓ 25 contributions
March: █████████▓▓ 30 contributions

Longest Streak: 3 months

🔵 GitHub Streak Information
────────────────────────────────────────
📅 Total Contributions: 80
🔥 Current Streak: 10 days
🏆 Longest Streak: 30 days
💪 Keep up the great work! You're on a roll!
```

## Contributing
Contributions are welcomed! Please open an issue or submit a pull request for any improvements or bug fixes.

##  License
This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
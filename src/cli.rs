use clap::{Arg, ArgAction, Command};

pub fn get_matches() -> clap::ArgMatches {
    Command::new("octotrack")
        .version("1.0")
        .author("Michal-python")
        .about("Github CLI activity viewer")
        .arg(
            Arg::new("username")
                .short('u')
                .long("username")
                .help("GitHub username")
                .required(true),
        )
        .arg(
            Arg::new("detailed")
                .short('d')
                .long("detailed")
                .help("Show detailed activity log")
                .action(ArgAction::SetTrue)
                .default_value("false"),
        )
        .arg(
            Arg::new("streak")
                .long("streak")
                .short('s')
                .help("Show contribution streaks")
                .action(ArgAction::SetTrue)
                .default_value("false"),
        )
        .arg(
            Arg::new("contributions")
                .long("contributions")
                .short('c')
                .help("Show contribution by months")
                .action(ArgAction::SetTrue)
                .default_value("false"),
        )
        .arg(
            Arg::new("json")
                .long("json")
                .help("Output in JSON format")
                .action(ArgAction::SetTrue)
                .default_value("false"),
        )
        .get_matches()
}

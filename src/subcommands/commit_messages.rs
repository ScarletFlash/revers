use clap::{Arg, App, AppSettings, ArgMatches};

pub fn get_app() -> App<'static> {
    return App::new("commit-messages")
        .about("Commit messages interaction: print all commit messages between commits or tags")
        .arg(
            Arg::new("from-commit")
                .long("from-commit")
                .takes_value(true)
                .value_name("COMMIT_SHA")
                .help("Specifies Git commit SHA to take messages from")
                .display_order(1)
        )
        .arg(
            Arg::new("to-commit")
                .long("to-commit")
                .takes_value(true)
                .value_name("COMMIT_SHA")
                .help("Specifies Git commit SHA to take messages to")
                .display_order(2)
        )
        .arg(
            Arg::new("from-tag")
                .long("from-tag")
                .takes_value(true)
                .value_name("TAG_NAME")
                .help("Specifies Git tag name to take messages from")
                .display_order(3)
        )
        .arg(
            Arg::new("to-tag")
                .long("to-tag")
                .takes_value(true)
                .value_name("TAG_NAME")
                .help("Specifies Git tag name to take messages to")
                .display_order(4)
        )
        .setting(AppSettings::ArgRequiredElseHelp)
        .display_order(2)
}

pub fn handle_matches(matches: &ArgMatches) -> () {
    match matches.subcommand() {
        Some(("commit-messages", _sub_matches)) => {
            println!("commit-messages");
        }
        _ => (),
    }
}

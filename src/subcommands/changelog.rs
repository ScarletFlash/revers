use clap::{Arg, App, AppSettings, ArgMatches};

pub fn get_app() -> App<'static> {
    return App::new("changelog")
        .about("Changelog interaction: generate changelog according to changes between commits or tags")
        .arg(
            Arg::new("from-commit")
                .long("from-commit")
                .takes_value(true)
                .value_name("COMMIT_SHA")
                .help("Specifies Git commit SHA to take commit messages from")
                .display_order(1)
        )
        .arg(
            Arg::new("to-commit")
                .long("to-commit")
                .takes_value(true)
                .value_name("COMMIT_SHA")
                .help("Specifies Git commit SHA to take commit messages to")
                .display_order(2)
        )
        .arg(
            Arg::new("from-tag")
                .long("from-tag")
                .takes_value(true)
                .value_name("TAG_NAME")
                .help("Specifies Git tag name to take commit messages from")
                .display_order(3)
        )
        .arg(
            Arg::new("to-tag")
                .long("to-tag")
                .takes_value(true)
                .value_name("TAG_NAME")
                .help("Specifies Git tag name to take commit messages to")
                .display_order(4)
        )
        .arg(
            Arg::new("create-file")
                .short('f')
                .long("create-file")
                .takes_value(true)
                .value_name("FILENAME")
                .help("Generates changelog if provided")
                .display_order(5)
        )
        .arg(
            Arg::new("output-dir")
                .long("output-dir")
                .takes_value(true)
                .value_name("PATH")
                .help("Specifies path for changelog generation")
                .display_order(6)
        )
        .setting(AppSettings::ArgRequiredElseHelp)
        .display_order(3)
}

pub fn handle_matches(matches: &ArgMatches) -> () {
    match matches.subcommand() {
        Some(("changelog", _sub_matches)) => {
            println!("changelog");
        }
        _ => (),
    }
}

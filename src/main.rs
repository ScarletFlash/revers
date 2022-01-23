#![deny(clippy::correctness)]
#![deny(clippy::suspicious)]
#![deny(clippy::style)]
#![deny(clippy::complexity)]
#![deny(clippy::perf)]
#![deny(clippy::pedantic)]
#![deny(clippy::nursery)]
#![deny(clippy::cargo)]
#![allow(clippy::single_match, clippy::unused_unit)]

use clap::{App, AppSettings, ArgMatches};

mod subcommands;
use subcommands::{changelog, commit_hashes, commit_messages, version};

fn main() {
  let matches: ArgMatches = App::new("revers")
    .bin_name("revers-cli")
    .about("CLI for customizable version management")
    .setting(AppSettings::SubcommandRequiredElseHelp)
    .setting(AppSettings::AllowExternalSubcommands)
    .setting(AppSettings::AllowInvalidUtf8ForExternalSubcommands)
    .setting(AppSettings::DisableHelpSubcommand)
    .setting(AppSettings::DisableVersionFlag)
    .subcommand_value_name("COMMAND")
    .subcommand_help_heading("COMMANDS")
    .subcommand(commit_hashes::get_app())
    .subcommand(changelog::get_app())
    .subcommand(commit_messages::get_app())
    .subcommand(version::get_app())
    .get_matches();

  commit_hashes::handle_matches(&matches);
  changelog::handle_matches(&matches);
  commit_messages::handle_matches(&matches);
  version::handle_matches(&matches);
}

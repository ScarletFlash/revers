use clap::{App, AppSettings, Arg, ArgMatches};
use git2::{Branch, Odb, Reference, Repository};
use std::env::current_dir;
use std::path::PathBuf;

pub fn get_app() -> App<'static> {
  return App::new("commit-hashes")
    .about("Commit hashes interaction: print all commit hashes between commits or tags")
    .arg(
      Arg::new("from-commit")
        .long("from-commit")
        .takes_value(true)
        .value_name("COMMIT_SHA")
        .help("Specifies Git commit SHA to take hashes from")
        .display_order(1),
    )
    .arg(
      Arg::new("to-commit")
        .long("to-commit")
        .takes_value(true)
        .value_name("COMMIT_SHA")
        .help("Specifies Git commit SHA to take hashes to")
        .display_order(2),
    )
    .arg(
      Arg::new("from-tag")
        .long("from-tag")
        .takes_value(true)
        .value_name("TAG_NAME")
        .help("Specifies Git tag name to take hashes from")
        .display_order(3),
    )
    .arg(
      Arg::new("to-tag")
        .long("to-tag")
        .takes_value(true)
        .value_name("TAG_NAME")
        .help("Specifies Git tag name to take hashes to")
        .display_order(4),
    )
    .setting(AppSettings::ArgRequiredElseHelp)
    .display_order(1);
}

pub fn handle_matches(matches: &ArgMatches) -> () {
  match matches.subcommand() {
    | Some(("commit-hashes", _sub_matches)) => get_commit_hashes(),
    | _ => (),
  }
}

fn get_commit_hashes() {
  let current_dir_path: PathBuf = current_dir().unwrap();
  let current_path: String = current_dir_path.to_str().unwrap().to_owned();

  let repository: Repository = match Repository::open(current_path) {
    | Ok(repository) => repository,
    | Err(error) => panic!("Git repository not found: {}", error),
  };

  let is_head_detached: bool = repository.head_detached().unwrap_or(true);
  assert!(!is_head_detached, "Can not access Git commits on detached HEAD");

  let head: Reference = repository.head().unwrap();
  assert!(head.is_branch(), "HEAD should be on branch to make commits interaction available");

  let branch: Branch = Branch::wrap(head);

  println!("{}", branch.name().unwrap().unwrap().to_owned());

  let _object_database: Odb = repository.odb().unwrap();
}

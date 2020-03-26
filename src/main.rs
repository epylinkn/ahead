extern crate clap;
extern crate regex;

use std::process::Command;
use clap::{Arg, App};
use regex::Regex;

fn main() -> Result<(), Box<dyn std::error::Error>> {
  let matches = App::new("Ahead")
    .version("0.1.0")
    .author("Anthony Bui")
    .about("Preview your day on the command line written in Rust")
    .arg(
      Arg::with_name("Days")
        .required(true)
        .takes_value(true)
        .index(1)
        .help("Number of days to preview")
    ).get_matches();

    // NB. Not using this yet...
    let days = matches.value_of("Days").unwrap();

    print_date();
    print_cal();
    print_icalbuddy();

    Ok(())
}

fn print_date() {
  let output = Command::new("date").output().expect("`date` failed");
  let result = String::from_utf8(output.stdout).unwrap();
  println!("{}", result);
}

fn print_cal() {
  let output = Command::new("cal").arg("-3").output().expect("`cal` failed");
  let result = String::from_utf8(output.stdout).unwrap();

  result.lines()
    .for_each(|x| println!("{}", x))
}

fn print_icalbuddy() {
  let output = Command::new("icalBuddy").arg("eventsToday").output().expect("icalBuddy failed");

  String::from_utf8(output.stdout)
    .unwrap()
    .lines()
    .for_each(|x| println!("{}", x));
}

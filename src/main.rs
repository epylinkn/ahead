extern crate clap;

use clap::{Arg, App};

fn main() {
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

    let days = matches.value_of("Days").unwrap();

    println!("{}", days);
}

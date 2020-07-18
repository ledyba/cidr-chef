/* coding: utf-8 */
/**
 * cidr-chef
 *
 * Copyright 2020-, Kaede Fujisaki
 */

extern crate clap;
use clap::{App, Arg, SubCommand, ArgMatches, AppSettings};
use std::process::exit;

mod cidr;

fn main() {
  let app = App::new("cidr-chef")
    .version("0.1.0")
    .author("Kaede Fujisaki <psi@7io.org>")
    .about("Swiss-Army Knife for CIDR calculation")
    .subcommand(SubCommand::with_name("calc")
      .setting(AppSettings::AllowLeadingHyphen)
      .arg(Arg::with_name("CIDR")
        .help("add CIDR set")
        .index(1)
        .takes_value(true)
        .multiple(true)
        .required(true)
        .allow_hyphen_values(true)
        .validator(|str| {
          if str.starts_with("-") || str.starts_with("+") {
            Ok(())
          }else {
            Err("-<addr> or +<addr>".to_string())
          }
        })
      )
    );
  let m = app.get_matches();
  match m.subcommand_name() {
    Some("calc") => {
      calc(m);
    }
    Some(x) => {
      eprint!("Unkown command: {}\n", x);
      exit(-1);
    }
    None => {
      eprint!("{}\n", m.usage());
      exit(-1);
    }
  }

}

fn calc(m: ArgMatches) {
  let mut cmds = m.subcommand_matches("calc").unwrap().values_of("CIDR").unwrap_or_default();
  for cmd in cmds {
    println!("{}", cmd);
  }
  cidr::Cidr::new("");
}

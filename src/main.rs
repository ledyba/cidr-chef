/* coding: utf-8 */
/**
 * cidr-chef
 *
 * Copyright 2020-, Kaede Fujisaki
 */

extern crate clap;
use clap::{App, Arg, SubCommand};
mod cidr;

fn main() {
  let app = App::new("cidr-chef")
    .version("0.1.0")
    .author("Kaede Fujisaki <psi@7io.org>")
    .about("Swiss-Army Knife for CIDR calculation")
    .subcommand(SubCommand::with_name("calc")
      .arg(Arg::with_name("CIDRS")
      .help("add CIDR set")
      .allow_hyphen_values(true)
      .multiple(true)
    ));
  let m = app.get_matches();
  let mut cmds = m.values_of("CIDRS").unwrap_or_default();
  while let cmd = cmds.next() {
    match cmd {
      Some("add") => {
        let set_opt = cmds.next();
        if set_opt.is_none() {
          eprint!("No CIDR set to add!");
          std::process::exit(-1);
        }
        let set = set_opt.unwrap();
        print!("{}", set);
      }
      Some("sub") => {
        let set_opt = cmds.next();
        if set_opt.is_none() {
          eprint!("No CIDR set to subtract!");
          std::process::exit(-1);
        }
        let set = set_opt.unwrap();
        print!("{}", set);
      }
      Some(s) => {
        eprint!("Unknwon command: {}", s);
        std::process::exit(-1);
      }
      None => {
        break;
      }
    }
  }
  cidr::Cidr::new("");
}

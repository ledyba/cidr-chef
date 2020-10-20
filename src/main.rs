/* coding: utf-8 */
/**
 * cidr-chef
 *
 * Copyright 2020-, Kaede Fujisaki
 */

extern crate clap;
use clap::{App, Arg, SubCommand, AppSettings};
use std::process::exit;

mod cidr;
mod cmds;

fn main() {
  let app = App::new("cidr-chef")
    .version("0.1.0")
    .author("Kaede Fujisaki <psi@7io.org>")
    .about("Swiss-Army Knife for CIDR calculation")
    .subcommand(SubCommand::with_name("calc")
      .setting(AppSettings::AllowLeadingHyphen)
      .arg(Arg::with_name("file")
        .help("CIDR set to add or subtract. ex) +0.0.0.0/0 -10.0.0.0/8")
        .long("from-file")
        .short("f")
        .allow_hyphen_values(true)
        .value_name("FILE or -(stdin)")
        .required(false))
      .arg(Arg::with_name("CIDR")
        .help("CIDR set to add or subtract. ex) +0.0.0.0/0 -10.0.0.0/8")
        .index(1)
        .takes_value(true)
        .multiple(true)
        .required(false)
        .allow_hyphen_values(true)
        .validator(|str| {
          if str.starts_with("-") || str.starts_with("+") {
            Ok(())
          }else {
            Err("-<addr> or +<addr>".to_string())
          }
        })))
      .subcommand(SubCommand::with_name("info")
        .arg(Arg::with_name("file")
          .help("IP addrs to get info")
          .long("from-file")
          .short("f")
          .multiple(true)
          .value_name("FILE or -(stdin)")
          .required(false))
        .arg(Arg::with_name("reports")
          .help("ALLOCATION AND ASSIGNMENT REPORTS")
          .long("report")
          .short("r")
          .multiple(true)
          .value_name("FILE")
          .required(true))
        .arg(Arg::with_name("ADDR")
          .help("IP addrs to get info")
          .index(1)
          .takes_value(true)
          .multiple(true)
          .required(false))
      );
  let m = app.get_matches();
  match m.subcommand_name() {
    Some("calc") => {
      if let Err(err) = cmds::calc::handle(&m) {
        eprint!("Failed to calc CIDR: {:?}\n", err);
        exit(-1);
      }
    }
    Some("info") => {
      if let Err(err) = cmds::info::handle(&m) {
        eprint!("Failed to get ip address info: {:?}\n", err);
        exit(-1);
      }
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



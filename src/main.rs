/* coding: utf-8 */
/**
 * cidr-chef
 *
 * Copyright 2020-, Kaede Fujisaki
 */

extern crate clap;
use clap::{App, Arg, SubCommand, ArgMatches, AppSettings};
use std::process::exit;
use crate::cidr::Cidr;

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
      if let Err(err) = calc(m) {
        eprint!("Failed to calc CIDR: {:?}\n", err);
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

fn calc(m: ArgMatches) -> Result<(), cidr::ParseError> {
  let cmds = m.subcommand_matches("calc").unwrap().values_of("CIDR").unwrap_or_default();
  let mut tree4 = cidr::tree::BitTree::new();
  let mut tree6 = cidr::tree::BitTree::new();
  for cmd in cmds {
    if cmd.starts_with("+") {
      let cidr = Cidr::new(&cmd[1..])?;
      match cidr.protocol {
        cidr::Protocol::IPv4 => tree4.add(&cidr),
        cidr::Protocol::IPv6 => tree6.add(&cidr)
      }
    } else if cmd.starts_with("-") {
      let cidr = Cidr::new(&cmd[1..])?;
      let success = match cidr.protocol {
        cidr::Protocol::IPv4 => tree4.sub(&cidr),
        cidr::Protocol::IPv6 => tree6.sub(&cidr)
      };
      if !success {
        eprintln!("{} is not contained.", &cmd[1..])
      }
    } else {
      panic!("{} is invalid cmd", cmd);
    }
  }
  if !tree4.is_empty() {
  }
  if !tree6.is_empty() {
  }
  Ok(())
}

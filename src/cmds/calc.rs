/* coding: utf-8 */
/******************************************************************************
 * cidr-chef
 *
 * Copyright 2020-, Kaede Fujisaki
 *****************************************************************************/

use clap::ArgMatches;
use crate::cidr;
use crate::cidr::Cidr;
use crate::cidr::tree::IpTree;
use std::io::BufRead;

fn handle_commands<'a, 'b, I, S>(tree4: &'a mut IpTree, tree6: &'a mut IpTree, cmds: I) -> Result<(), cidr::ParseError>
where
  I: Iterator<Item=S>,
  S: AsRef<str>
{
  for it in cmds {
    let cmd: &str = it.as_ref();
    match cmd {
      cmd if cmd.starts_with("+") => {
        let cidr = Cidr::parse(&cmd[1..])?;
        match cidr.protocol {
          cidr::Protocol::IPv4 => tree4.add(&cidr),
          cidr::Protocol::IPv6 => tree6.add(&cidr),
        }
      }
      cmd if cmd.starts_with("-") => {
        let cidr = Cidr::parse(&cmd[1..])?;
        let success = match cidr.protocol {
          cidr::Protocol::IPv4 => tree4.sub(&cidr),
          cidr::Protocol::IPv6 => tree6.sub(&cidr),
        };
        if !success {
          eprintln!("{} is not fully contained in current set.", &cmd[1..])
        }
      }
      cmd => {
        let cidr_try = Cidr::parse(cmd);
        if cidr_try.is_err() {
          return Err(cidr_try.unwrap_err());
        }
        let cidr = cidr_try.unwrap();
        match cidr.protocol {
          cidr::Protocol::IPv4 => tree4.add(&cidr),
          cidr::Protocol::IPv6 => tree6.add(&cidr),
        }
      }
    }
  }
  Ok(())
}

pub fn handle(m: ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
  let mut tree4 = cidr::tree::IpTree::new();
  let mut tree6 = cidr::tree::IpTree::new();

  let m = m.subcommand_matches("calc").unwrap();
  if m.value_of("file").is_some() { // Handle file first
    let path = m.value_of("file").unwrap();
    let trans = |item: Result<std::string::String, _>| -> std::vec::Vec<std::string::String> {
      match item {
        Ok(string) => {
          let str = string.trim();
          if str.len() > 0 && str.chars().nth(0) != Some('#') {
            vec![str.to_string()]
          } else {
            vec![]
          }
        },
        Err(_) => vec![],
      }
    };
    if path == "-" {
      handle_commands(&mut tree4, &mut tree6, std::io::BufReader::new(std::io::stdin()).lines().flat_map(trans))?;
    } else {
      let file = std::fs::File::open(path)?;
      handle_commands(&mut tree4, &mut tree6, std::io::BufReader::new(file).lines().flat_map(trans))?;
    };
  }
  {
    let cmds = m.values_of("CIDR").unwrap_or_default();
    handle_commands(&mut tree4, &mut tree6, cmds.into_iter())?;
  }
  if !tree4.is_empty() {
    for cidr in tree4.extract4() {
      println!("{}", cidr.to_string());
    }
  }
  if !tree6.is_empty() {
    for cidr in tree4.extract6() {
      println!("{}", cidr.to_string());
    }
  }
  Ok(())
}

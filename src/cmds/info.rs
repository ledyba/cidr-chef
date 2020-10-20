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
use std::collections::HashSet;
use std::io;

pub fn handle(m: &ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
  let mut tree4 = cidr::tree::IpTree::<()>::new();
  let mut tree6 = cidr::tree::IpTree::<()>::new();

  let mut addrs: HashSet<String> = HashSet::new();

  let m = m.subcommand_matches("info").unwrap();
  if let Some(path) = m.value_of("file") { // Handle file first
    let trans = |mut addrs: HashSet<String>, item: Result<std::string::String, _>| -> io::Result<HashSet<String>> {
      match item {
        Ok(addr) => {
          addrs.insert(addr.trim().to_string());
          Ok(addrs)
        },
        Err(err) => Err(err),
      }
    };
    match path {
      "-" => {
        for line in std::io::BufReader::new(std::io::stdin()).lines() {
          addrs.insert(line?.trim().to_string());
        }
      }
      fpath => {
        let file = std::fs::File::open(fpath)?;
        for line in std::io::BufReader::new(file).lines() {
          addrs.insert(line?.trim().to_string());
        }
      },
    };
  }
  for addr in  m.values_of("ADDR").unwrap_or_default().into_iter() {
    addrs.insert(addr.trim().to_string());
  }
  for addr in addrs {
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

/* coding: utf-8 */
/**
 * cidr-chef
 *
 * Copyright 2020-, Kaede Fujisaki
 */

pub mod tree;

use std::num::ParseIntError;

#[derive(Debug, PartialEq)]
pub struct Cidr {
  address: u128,
  len: usize,
}

#[derive(Debug, PartialEq)]
pub enum ParseError {
  ParseIntError(ParseIntError),
  ParseCidrError(String),
}

pub type ParseResult = std::result::Result<Cidr, ParseError>;

impl std::convert::From<std::num::ParseIntError> for ParseError {
  fn from(err: ParseIntError) -> Self {
    ParseError::ParseIntError(err)
  }
}


impl Cidr {
  pub fn new(repr: &str) -> ParseResult {
    if repr.contains(".") {
      parse4(repr, repr, 0, 32)
    } else {
      parse6(repr, repr)
    }
  }
}

fn parse6(all: &str, repr: &str) -> ParseResult {
  let double_colon_pos = all.find("::");
  let slash_pos = all.find("/");
  if slash_pos.is_none() {
    return Err(ParseError::ParseCidrError(format!("{} does not contain valid network address", all)))
  }
  let len = all[slash_pos.unwrap()+1..].parse::<usize>();
  if double_colon_pos.is_none() {
    let (addr, addr_len) = parse6_body(all, &repr[..slash_pos.unwrap()], 0, 0)?;
    if addr_len != 128 {
      return Err(ParseError::ParseCidrError(format!("{} is too short.", all)))
    }
    return Ok(Cidr{
      address: addr,
      len: len.unwrap(),
    });
  }
  let first = &repr[..double_colon_pos.unwrap()];
  let second = &repr[double_colon_pos.unwrap()+2..slash_pos.unwrap()];
  let (first_addr, first_len) = parse6_body(first, first, 0, 0)?;
  let (second_addr, second_len) = parse6_body(second, second, 0, 0)?;
  if first_len + second_len > 128 {
    return Err(ParseError::ParseCidrError(format!("{} is too long.", all)))
  }
  Ok(Cidr {
    address: (first_addr << (128-first_len)) | second_addr,
    len: len.unwrap(),
  })
}
fn parse6_body(all: &str, repr: &str, acc: u128, size: usize) -> Result<(u128, usize), ParseError> {
  let sep_pos = repr.find(":");
  match sep_pos{
    Some(sep_pos) => {
      let part = repr[0..sep_pos].parse::<u16>()?;
      let next_acc = (acc << 16) | part as u128;
      let next_size = size + 16;
      parse6_body(all, &repr[sep_pos+1..], next_acc, next_size)
    }
    None => {
      let part = repr.parse::<u16>()?;
      Ok(((acc << 16) | part as u128, size + 16))
    }
  }
}

fn parse4(all: &str, repr: &str, acc: u32, pos: usize) -> ParseResult {
  let next_pos = pos - 8;

  let sep_pos = repr.find(if next_pos > 0 { "." } else { "/" });
  if sep_pos.is_none() {
    return Err(ParseError::ParseCidrError(format!("{} does not contain valid network address", all)))
  }
  let byte = &repr[0..sep_pos.unwrap()].parse::<u32>()?;
  let next_acc = acc | (byte << next_pos);
  if next_pos == 0 {
    let len = &repr[sep_pos.unwrap()+1..].parse::<usize>()?;
    Ok(Cidr {
      address: next_acc as u128,
      len: len.clone(),
    })
  } else {
    parse4(all, &repr[sep_pos.unwrap() + 1..], next_acc, next_pos)
  }
}

#[test]
fn parse_ipv4() {
  assert_eq!(Cidr::new("1.2.3.4/12"), Ok(Cidr{ address: 0x01020304, len: 12}) as ParseResult);
}

#[test]
fn parse_ipv6() {
  assert_eq!(Cidr::new("1::2/61"), Ok(Cidr{ address: 0x0001_0000_0000_0000_0000_0000_0000_0002, len: 61}) as ParseResult);
}

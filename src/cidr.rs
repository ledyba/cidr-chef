/* coding: utf-8 */
/**
 * cidr-chef
 *
 * Copyright 2020-, Kaede Fujisaki
 */

pub mod tree;

use std::num::ParseIntError;
use clap::Format::Error;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Protocol {
  IPv4,
  IPv6,
}

#[derive(Debug, PartialEq)]
pub struct Cidr {
  pub protocol: Protocol,
  pub address: u128,
  pub bits: usize,
}

#[derive(Debug, PartialEq)]
pub enum ParseError {
  FailedToParseInt(ParseIntError),
  FailedToParseCidr(String),
  FailedToDetectIpVersion,
}

impl std::fmt::Display for ParseError {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    match self {
      ParseError::FailedToParseInt(err) => {
        write!(f, "Failed to parse as an integer: {:?}", err)
      }
      ParseError::FailedToParseCidr(err) => {
        write!(f, "Failed to parse as a CIDR representation: {:?}", err)
      }
      ParseError::FailedToDetectIpVersion => {
        f.write_str("Failed to detect ip version")
      }
    }
  }
}

impl std::error::Error for ParseError {}

pub type ParseResult = std::result::Result<Cidr, ParseError>;

impl std::convert::From<std::num::ParseIntError> for ParseError {
  fn from(err: ParseIntError) -> Self {
    ParseError::FailedToParseInt(err)
  }
}

impl Cidr {
  pub fn parse(addr: &str) -> ParseResult {
    if addr.contains(".") {
      parse4(addr, addr, 0, 32)
    } else if addr.contains(":") {
      parse6(addr, addr)
    } else {
      Error(ParseError::CannotDetectAddrVersionError)
    }
  }
  fn to_string4(&self) -> String {
    format!("{}.{}.{}.{}/{}", (self.address >> 24 & 0xff), (self.address >> 16 & 0xff), (self.address >> 8 & 0xff), (self.address >> 0 & 0xff), self.bits)
  }
  fn to_string6(&self) -> String {
    "".to_string()
  }
}

impl ToString for Cidr {
  fn to_string(&self) -> String {
    match self.protocol {
      Protocol::IPv4 => self.to_string4(),
      Protocol::IPv6 => self.to_string6()
    }
  }
}

fn parse6(all: &str, repr: &str) -> ParseResult {
  let double_colon_pos = all.find("::");
  let slash_pos = all.find("/");
  if slash_pos.is_none() {
    return Err(ParseError::FailedToParseCidr(format!("{} does not contain valid network address", all)))
  }
  let len = all[slash_pos.unwrap()+1..].parse::<usize>();
  if double_colon_pos.is_none() {
    let (addr, addr_len) = parse6_body(all, &repr[..slash_pos.unwrap()], 0, 0)?;
    if addr_len != 128 {
      return Err(ParseError::FailedToParseCidr(format!("{} is too short.", all)))
    }
    return Ok(Cidr{
      protocol: Protocol::IPv6,
      address: addr,
      bits: len.unwrap(),
    });
  }
  let first = &repr[..double_colon_pos.unwrap()];
  let second = &repr[double_colon_pos.unwrap()+2..slash_pos.unwrap()];
  let (first_addr, first_len) =
    if first.is_empty() {
      (0, 0)
    } else {
      parse6_body(first, first, 0, 0)?
    };
  let (second_addr, second_len) =
    if second.is_empty() {
      (0, 0)
    } else {
      parse6_body(second, second, 0, 0)?
    };
  if first_len + second_len > Protocol::IPv6.len() {
    return Err(ParseError::FailedToParseCidr(format!("{} is too long.", all)))
  }
  let address =
    if first_len == 0 {
      second_addr
    } else {
      (first_addr << (Protocol::IPv6.len() - first_len)) | second_addr
    };
  Ok(Cidr {
    protocol: Protocol::IPv6,
    address,
    bits: len.unwrap(),
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
    return Err(ParseError::FailedToParseCidr(format!("{} does not contain valid network address", all)))
  }
  let byte = &repr[0..sep_pos.unwrap()].parse::<u32>()?;
  let next_acc = acc | (byte << next_pos);
  if next_pos == 0 {
    let len = &repr[sep_pos.unwrap()+1..].parse::<usize>()?;
    Ok(Cidr {
      protocol: Protocol::IPv4,
      address: next_acc as u128,
      bits: len.clone(),
    })
  } else {
    parse4(all, &repr[sep_pos.unwrap() + 1..], next_acc, next_pos)
  }
}

impl Protocol {
  pub fn len(&self) -> usize {
    match self {
      Protocol::IPv4 => 32,
      Protocol::IPv6 => 128,
    }
  }
}

#[test]
fn parse_ipv4() {
  assert_eq!(Cidr::parse("1.2.3.4/12"), Ok(Cidr{ protocol: Protocol::IPv4, address: 0x01020304, bits: 12}) as ParseResult);
}

#[test]
fn parse_ipv6() {
  assert_eq!(Cidr::parse("1::2/61"), Ok(Cidr{ protocol: Protocol::IPv6, address: 0x0001_0000_0000_0000_0000_0000_0000_0002, bits: 61}) as ParseResult);
  assert_eq!(Cidr::parse("1::/61"), Ok(Cidr{ protocol: Protocol::IPv6, address: 0x0001_0000_0000_0000_0000_0000_0000_0000, bits: 61}) as ParseResult);
  assert_eq!(Cidr::parse("0::2/61"), Ok(Cidr{ protocol: Protocol::IPv6, address: 0x0000_0000_0000_0000_0000_0000_0000_0002, bits: 61}) as ParseResult);
  assert_eq!(Cidr::parse("::/61"), Ok(Cidr{ protocol: Protocol::IPv6, address: 0x0, bits: 61}) as ParseResult);
  assert_eq!(Cidr::parse("::/61"), Ok(Cidr{ protocol: Protocol::IPv6, address: 0x0, bits: 61}) as ParseResult);
}

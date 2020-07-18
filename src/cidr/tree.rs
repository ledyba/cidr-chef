/* coding: utf-8 */
use crate::cidr::{Cidr, Protocol};

/**
 * cidr-chef
 *
 * Copyright 2020-, Kaede Fujisaki
 */

#[derive(Default, Debug)]
pub struct BitTree {
  root: Option<Box<Tree>>
}

#[derive(Default, Debug)]
struct Tree {
  children: [Option<Box<Tree>>; 2]
}

impl Tree {
  pub fn is_end(&self) -> bool {
    self.children.iter().all(Option::is_none)
  }
}

impl BitTree {
  // create empty tree.
  pub fn new() -> BitTree {
    return BitTree {
      root: None,
    }
  }
  pub fn is_empty(&self) -> bool {
    return self.root.is_none();
  }
  pub fn add(&mut self, cidr: &Cidr) {
    match cidr.protocol {
      Protocol::IPv4 => add_mask(&mut self.root,cidr.address << (128-32), cidr.bits),
      Protocol::IPv6 => add_mask(&mut self.root,cidr.address, cidr.bits)
    };
  }
  pub fn sub(&mut self, cidr: &Cidr) -> bool {
    match cidr.protocol {
      Protocol::IPv4 => sub_mask(&mut self.root, cidr.address << (128-32), cidr.bits),
      Protocol::IPv6 => sub_mask(&mut self.root, cidr.address, cidr.bits)
    }
  }
  fn extract(&self, protocol: Protocol) -> Vec<Cidr> {
    let mut acc = Vec::<Cidr>::new();
    extract(protocol, &self.root, 0, 0, &mut acc);
    acc
  }
  pub fn extract4(&self) -> Vec<Cidr> {
    self.extract(Protocol::IPv4)
  }
  pub fn extract6(&self) -> Vec<Cidr> {
    self.extract(Protocol::IPv6)
  }
}

fn add_mask(curr: &mut Option<Box<Tree>>, mask: u128, bits: usize) -> bool {
  if curr.is_some() && curr.as_ref().unwrap().is_end() {
    return true;
  }
  if bits == 0 {
    *curr = Some(Box::default());
    return true;
  }
  let b = (mask >> 127 & 1) as usize;
  if curr.is_none() {
    *curr = Some(Box::default());
  }
  let next: &mut Option<Box<Tree>> = &mut curr.as_mut().unwrap().children[b];
  if add_mask(next, mask << 1, bits - 1) {
    let other = &curr.as_ref().unwrap().children[(b+1)%2];
    if other.is_some() && other.as_ref().unwrap().is_end() {
      *curr = Some(Box::default());
      true
    } else {
      false
    }
  } else {
    false
  }
}

fn sub_mask(curr_opt: &mut Option<Box<Tree>>,mask: u128, bits: usize) -> bool {
  if bits == 0 {
    *curr_opt = None;
    return true;
  }
  if curr_opt.is_none() {
    return false;
  }
  let b = (mask >> 127 & 1) as usize;
  let curr = curr_opt.as_mut().unwrap();
  if curr.children.iter().any(Option::is_some) {
    sub_mask(&mut curr.children[b], mask << 1, bits - 1);
    if curr.children.iter().all(Option::is_none) {
      *curr_opt = None;
    }
  } else {
    curr.children[(b+1)%2] = Some(Box::default());
    curr.children[b] = None;
  }
  true
}

fn extract(protocol: Protocol, curr: &Option<Box<Tree>>, acc: u128, depth: usize, vec: &mut Vec<Cidr>) {
  match curr {
    Some(curr) => {
      if curr.children.iter().all(Option::is_none) {
        vec.push(Cidr{
          protocol,
          address: acc << (protocol.len() - depth),
          bits: depth,
        });
      }
      for (i, child) in curr.children.iter().enumerate() {
        extract(protocol, child, (acc << 1) | i as u128, depth + 1, vec);
      }
    }
    None => {}
  }
}

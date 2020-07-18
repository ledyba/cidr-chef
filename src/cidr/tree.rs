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
    }
  }
  pub fn sub(&mut self, cidr: &Cidr) -> bool {
    match cidr.protocol {
      Protocol::IPv4 => sub_mask(&mut self.root, cidr.address << (128-32), cidr.bits),
      Protocol::IPv6 => sub_mask(&mut self.root, cidr.address, cidr.bits)
    }
  }
}

fn add_mask(curr: &mut Option<Box<Tree>>, mask: u128, bits: usize) {
  if curr.is_some() {
    return;
  }
  if bits == 0 {
    *curr = Some(Box::default());
    return;
  }
  *curr = Some(Box::default());
  let b = mask >> 127 & 1;
  let next: &mut Option<Box<Tree>> = &mut curr.as_mut().unwrap().children[b as usize];
  add_mask(next, mask << 1, bits - 1)
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
  let curr = &mut curr_opt.as_ref().unwrap();
  if curr.any(Option::is_some) {
    sub_mask(&mut curr.children[b], mask << 1, bits - 1)
  } else {
    curr.children[(b+1)%2] = Some(Box::default());
    curr.children[b] = None;
    true
  }
}
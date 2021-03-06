/* coding: utf-8 */
/******************************************************************************
 * cidr-chef
 *
 * Copyright 2020-, Kaede Fujisaki
 *****************************************************************************/
use crate::cidr::{Cidr, Protocol};

#[derive(Default, Debug)]
pub struct IpTree<V> {
  root: Option<Box<Node<V>>>
}

#[derive(Debug)]
struct Node<V> where {
  value: Option<V>,
  branches: [Option<Box<Node<V>>>; 2]
}

impl<V> Default for Node<V> {
  fn default() -> Self {
    Node {
      value: None,
      branches: [None, None],
    }
  }
}

impl<V> Node<V> {
  pub fn is_leaf(&self) -> bool {
    self.branches.iter().all(Option::is_none)
  }
}

impl<V> IpTree<V> {
  // create empty tree.
  pub fn new() -> IpTree<V> {
    return IpTree {
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

fn add_mask<V>(curr: &mut Option<Box<Node<V>>>, mask: u128, bits: usize) {
  if curr.is_some() && curr.as_ref().unwrap().is_leaf() {
    return;
  }
  if bits == 0 {
    *curr = Some(Box::default());
    return;
  }
  let b = (mask >> 127 & 1) as usize;
  if curr.is_none() {
    *curr = Some(Box::default());
  }
  let next: &mut Option<Box<Node<V>>> = &mut curr.as_mut().unwrap().branches[b];
  add_mask(next, mask << 1, bits - 1);
  if let Some(tree) = curr {
    if tree.branches.iter().all(|child| child.is_some() && child.as_ref().unwrap().is_leaf()) {
      *curr = Some(Box::default());
    }
  }
}

fn sub_mask<V>(curr_opt: &mut Option<Box<Node<V>>>, mask: u128, bits: usize) -> bool {
  if bits == 0 {
    let result = curr_opt.as_ref().map_or_else(|| false, |it| it.is_leaf());
    *curr_opt = None;
    return result;
  }
  if curr_opt.is_none() {
    return false;
  }
  let curr = curr_opt.as_mut().unwrap();
  let b = (mask >> 127 & 1) as usize;
  let mut split = false;
  if curr.is_leaf() {
    curr.branches[0] = Some(Box::default());
    curr.branches[1] = Some(Box::default());
    split = true;
  }
  let result = sub_mask(&mut curr.branches[b], mask << 1, bits - 1);
  if curr.is_leaf() {
    *curr_opt = None;
  }
  split || result
}

fn extract<V>(protocol: Protocol, curr: &Option<Box<Node<V>>>, acc: u128, depth: usize, vec: &mut Vec<Cidr>) {
  match curr {
    Some(curr) => {
      if curr.branches.iter().all(Option::is_none) {
        vec.push(Cidr{
          protocol,
          address: acc << (protocol.len() - depth),
          bits: depth,
        });
      }
      for (i, child) in curr.branches.iter().enumerate() {
        extract(protocol, child, (acc << 1) | i as u128, depth + 1, vec);
      }
    }
    None => {}
  }
}

#[test]
fn test_simple() {
  {
    let mut t = IpTree::<()>::new();
    t.add(&Cidr::parse("192.168.0.0/24").unwrap());
    assert!(t.sub(&Cidr::parse("192.168.0.0/24").unwrap()));
    assert!(t.is_empty());
  }
  {
    let mut t = IpTree::<()>::new();
    t.add(&Cidr::parse("0.0.0.0/0").unwrap());
    assert!(t.sub(&Cidr::parse("192.168.0.0/24").unwrap()));
    t.add(&Cidr::parse("192.168.0.0/24").unwrap());
    let lst = t.extract4();
    assert_eq!(lst.len(), 1);
    assert_eq!(lst[0].to_string(), "0.0.0.0/0");
  }
}

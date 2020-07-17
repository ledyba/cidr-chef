/* coding: utf-8 */
/**
 * cidr-chef
 *
 * Copyright 2020-, Kaede Fujisaki
 */

pub struct BitTree {
  include_all: bool,
  children: std::vec::Vec<BitTree>,
}

impl BitTree {
  // create empty tree.
  fn new() -> BitTree {
    return BitTree {
      include_all: false,
      children: vec![],
    }
  }

  fn add(&mut self) {

  }
}

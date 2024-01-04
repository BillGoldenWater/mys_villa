/*
 * Copyright 2023 - Golden_Water
 *
 * SPDX-License-Identifier: MIT
 */

use std::{
  fmt::{Display, Formatter},
  ops::Deref,
};

#[derive(Debug, Copy, Clone)]
pub struct StatusCode(u16);

impl StatusCode {
  pub fn new(code: u16) -> Self {
    Self(code)
  }
}

impl Display for StatusCode {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.0)
  }
}

impl Deref for StatusCode {
  type Target = u16;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl PartialEq<u16> for StatusCode {
  fn eq(&self, other: &u16) -> bool {
    self.0.eq(other)
  }
}

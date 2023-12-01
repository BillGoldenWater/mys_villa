/*
 * Copyright 2023 - Golden_Water
 *
 * SPDX-License-Identifier: MIT
 */

use std::ops::Deref;

use crate::http::request::body::multipart::part::Part;

pub mod part;

#[derive(Debug, Clone, Default)]
pub struct Multipart(Vec<(String, Part)>);

impl Multipart {
  pub fn new(parts: Vec<(String, Part)>) -> Self {
    Self(parts)
  }
}

impl Deref for Multipart {
  type Target = Vec<(String, Part)>;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl IntoIterator for Multipart {
  type Item = (String, Part);
  type IntoIter = std::vec::IntoIter<Self::Item>;

  fn into_iter(self) -> Self::IntoIter {
    self.0.into_iter()
  }
}

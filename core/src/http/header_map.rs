/*
 * Copyright 2023 - Golden_Water
 *
 * SPDX-License-Identifier: MIT
 */

use std::collections::HashMap;
use std::ops::{Deref, DerefMut};

#[derive(Debug, Clone, Default)]
pub struct HeaderMap(HashMap<String, String>);

impl HeaderMap {
  pub fn new(headers: HashMap<String, String>) -> Self {
    Self(headers)
  }
}

impl Deref for HeaderMap {
  type Target = HashMap<String, String>;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl DerefMut for HeaderMap {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.0
  }
}

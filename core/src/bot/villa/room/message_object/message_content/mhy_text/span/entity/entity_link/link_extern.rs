/*
 * Copyright 2023 - Golden_Water
 *
 * SPDX-License-Identifier: MIT
 */

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct LinkExtern {
  pub url: String,
  pub require_access_token: bool,
}

impl LinkExtern {
  pub fn new(url: String, require_access_token: bool) -> Self {
    Self {
      url,
      require_access_token,
    }
  }
}

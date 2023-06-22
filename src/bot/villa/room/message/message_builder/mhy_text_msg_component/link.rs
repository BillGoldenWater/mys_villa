/*
 * Copyright 2023 - Golden_Water
 *
 * SPDX-License-Identifier: MIT
 */

/// link
#[derive(Debug, Clone, PartialEq)]
pub struct Link {
  /// link name
  pub link_name: String,
  /// link url
  pub url: String,
}

impl Link {
  /// initialize with link_name and url
  pub fn new(link_name: impl Into<String>, url: impl Into<String>) -> Self {
    Self {
      link_name: link_name.into(),
      url: url.into(),
    }
  }
}

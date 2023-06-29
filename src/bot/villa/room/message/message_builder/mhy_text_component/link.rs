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
  /// require bot access token when member access the link
  pub requires_bot_access_token: bool,
}

impl Link {
  /// initialize with link_name, url and requires_bot_access_token
  pub fn new(
    link_name: impl Into<String>,
    url: impl Into<String>,
    requires_bot_access_token: bool,
  ) -> Self {
    Self {
      link_name: link_name.into(),
      url: url.into(),
      requires_bot_access_token,
    }
  }
}

/*
 * Copyright 2023 - Golden_Water
 *
 * SPDX-License-Identifier: MIT
 */

/// mention user
#[derive(Debug, Clone, PartialEq)]
pub struct MentionUser {
  /// user name
  pub user_name: String,
  /// user id
  pub uid: u64,
}

impl MentionUser {
  /// initialize with user name and uid
  pub fn new(user_name: impl Into<String>, uid: u64) -> Self {
    Self {
      user_name: user_name.into(),
      uid,
    }
  }
}

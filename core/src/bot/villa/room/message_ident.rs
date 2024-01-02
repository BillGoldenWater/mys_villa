/*
 * Copyright 2023 - Golden_Water
 *
 * SPDX-License-Identifier: MIT
 */

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct MessageIdent {
  pub id: String,
  pub send_at: i64,
}

impl MessageIdent {
  pub fn new(id: String, send_at: i64) -> Self {
    Self { id, send_at }
  }
}

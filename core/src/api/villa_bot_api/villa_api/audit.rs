/*
 * Copyright 2023 - Golden_Water
 *
 * SPDX-License-Identifier: MIT
 */

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct AuditRequest {
  pub uid: u64,
  pub room_id: Option<u64>,
  pub content_type: String,
  pub audit_content: String,
  pub pass_through: Option<String>,
}

impl AuditRequest {
  pub fn new(
    uid: u64,
    room_id: Option<u64>,
    content_type: String,
    audit_content: String,
    pass_through: Option<String>,
  ) -> Self {
    Self {
      uid,
      room_id,
      content_type,
      audit_content,
      pass_through,
    }
  }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct AuditResponse {
  pub audit_id: String,
}

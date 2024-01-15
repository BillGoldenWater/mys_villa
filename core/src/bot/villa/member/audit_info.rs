/*
 * Copyright 2023 - Golden_Water
 *
 * SPDX-License-Identifier: MIT
 */

use typed_builder::TypedBuilder;

use crate::{
  api::villa_bot_api::villa_api::member_api::audit::AuditRequest,
  bot::villa::member::audit_info::audit_content::AuditContent,
};

pub mod audit_content;

#[derive(Debug, Clone, PartialEq, TypedBuilder)]
pub struct AuditInfo {
  #[builder(default, setter(strip_option))]
  pub room_id: Option<u64>,
  #[builder(default, setter(strip_option))]
  pub pass_through: Option<String>,
  pub content: AuditContent,
}

impl AuditInfo {
  pub fn construct_req(self, uid: u64) -> AuditRequest {
    AuditRequest {
      uid,
      room_id: self.room_id,
      content_type: self.content.get_type().into(),
      audit_content: self.content.to_content(),
      pass_through: self.pass_through,
    }
  }
}

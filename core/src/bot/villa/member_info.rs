/*
 * Copyright 2023 - Golden_Water
 *
 * SPDX-License-Identifier: MIT
 */

use crate::api::villa_bot_api::villa_api::member::member_basic::MemberBasic;
use crate::api::villa_bot_api::villa_api::member::Member;
use crate::bot::villa::role_info::RoleInfo;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct MemberInfo {
  pub id: u64,
  pub nickname: String,
  pub introduce: String,
  pub avatar_id: Option<u64>,
  pub avatar_url: String,
  // ---
  pub joined_at: u64,
  pub roles: Vec<RoleInfo>,
}

impl From<Member> for MemberInfo {
  fn from(value: Member) -> Self {
    Self {
      id: value.basic.uid,
      nickname: value.basic.nickname,
      introduce: value.basic.introduce,
      avatar_id: value.basic.avatar,
      avatar_url: value.basic.avatar_url,
      joined_at: value.joined_at,
      roles: value.role_list.into_iter().map(|v| v.into()).collect(),
    }
  }
}

impl From<MemberInfo> for Member {
  fn from(value: MemberInfo) -> Self {
    Self {
      basic: MemberBasic {
        uid: value.id,
        nickname: value.nickname,
        introduce: value.introduce,
        avatar: value.avatar_id,
        avatar_url: value.avatar_url,
      },
      role_id_list: value.roles.iter().map(|v| v.id).collect(),
      joined_at: value.joined_at,
      role_list: value.roles.into_iter().map(|v| v.into()).collect(),
    }
  }
}

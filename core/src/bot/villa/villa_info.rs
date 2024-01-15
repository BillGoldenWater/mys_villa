/*
 * Copyright 2023 - Golden_Water
 *
 * SPDX-License-Identifier: MIT
 */

use crate::api::villa_bot_api::villa_api;

#[derive(Debug, Clone, PartialEq)]
pub struct VillaInfo {
  pub id: u64,
  pub owner_uid: u64,

  pub name: String,
  pub introduce: String,
  pub avatar_url: String,

  pub is_official: bool,

  pub category_id: u32,
  pub tags: Vec<String>,
}

impl From<villa_api::villa_info::VillaInfo> for VillaInfo {
  fn from(value: villa_api::villa_info::VillaInfo) -> Self {
    Self {
      id: value.villa_id,
      owner_uid: value.owner_uid,

      name: value.name,
      introduce: value.introduce,
      avatar_url: value.villa_avatar_url,

      is_official: value.is_official,

      category_id: value.category_id,
      tags: value.tags,
    }
  }
}

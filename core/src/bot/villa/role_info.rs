/*
 * Copyright 2023 - Golden_Water
 *
 * SPDX-License-Identifier: MIT
 */

use crate::{
  api::villa_bot_api::villa_api::member_role::MemberRole,
  bot::villa::role_info::{
    role_color::RoleColor, role_permission_info::RolePermissionInfo, role_type::RoleType,
  },
};

pub mod role_color;
pub mod role_permission_info;
pub mod role_type;

#[derive(Debug, Clone, PartialEq)]
pub struct RoleInfo {
  pub id: u64,
  pub villa_id: u64,
  pub name: String,
  pub color: RoleColor,
  pub r#type: RoleType,
  pub is_all_room: bool,
  pub room_ids: Vec<u64>,
  pub permissions: Vec<RolePermissionInfo>,
  // --- undocumented
  pub web_color: String,
  pub bg_color: String,
  pub color_scheme_id: i64,
  pub font_color: String,
  pub has_manage_perm: Option<bool>,
  pub member_num: Option<u64>,
  pub priority: i64,
}

impl From<MemberRole> for RoleInfo {
  fn from(value: MemberRole) -> Self {
    Self {
      id: value.id,
      villa_id: value.villa_id,
      name: value.name,
      color: value.color.into(),
      r#type: value.role_type.into(),
      is_all_room: value.is_all_room,
      room_ids: value.room_ids,
      permissions: value.permissions.into_iter().map(Into::into).collect(),
      web_color: value.web_color,
      bg_color: value.bg_color,
      color_scheme_id: value.color_scheme_id,
      font_color: value.font_color,
      has_manage_perm: value.has_manage_perm,
      member_num: value.member_num,
      priority: value.priority,
    }
  }
}

impl From<RoleInfo> for MemberRole {
  fn from(value: RoleInfo) -> Self {
    Self {
      id: value.id,
      name: value.name,
      villa_id: value.villa_id,
      color: value.color.into(),
      role_type: value.r#type.into(),
      is_all_room: value.is_all_room,
      room_ids: value.room_ids,
      permissions: value.permissions.into_iter().map(Into::into).collect(),
      web_color: value.web_color,
      bg_color: value.bg_color,
      color_scheme_id: value.color_scheme_id,
      font_color: value.font_color,
      has_manage_perm: value.has_manage_perm,
      member_num: value.member_num,
      priority: value.priority,
    }
  }
}

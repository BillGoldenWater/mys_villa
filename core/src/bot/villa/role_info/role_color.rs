/*
 * Copyright 2023 - Golden_Water
 *
 * SPDX-License-Identifier: MIT
 */

use crate::impl_enum_str_convert;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum RoleColor {
  /// #EB7F89
  Owner,
  /// #8F9BBF
  AllMember,
  /// #6173AB
  DarkBlue,
  /// #F485D8
  LightPink,
  /// #F47884
  Red,
  /// #FFA54B
  Orange,
  /// #7BC26F
  LightGreen,
  /// #59A1EA
  LightBlue,
  /// #977EE1
  Purple,
  /// unknown color
  Unknown(String),
}

impl_enum_str_convert! {
  RoleColor;
  @preprocess_fn(value) => {value.to_ascii_uppercase()};
  "#EB7F89" => Owner,
  "#8F9BBF" => AllMember,
  "#6173AB" => DarkBlue,
  "#F485D8" => LightPink,
  "#F47884" => Red,
  "#FFA54B" => Orange,
  "#7BC26F" => LightGreen,
  "#59A1EA" => LightBlue,
  "#977EE1" => Purple;
  _ => Unknown,
}

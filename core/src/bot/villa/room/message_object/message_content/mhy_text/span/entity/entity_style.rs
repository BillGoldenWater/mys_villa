/*
 * Copyright 2023 - Golden_Water
 *
 * SPDX-License-Identifier: MIT
 */

use crate::api::villa_bot_api::villa_api::room_api::msg_content_info::msg_content::msg_mhy_text::mhy_text_entity::entity_data::font_style::FontStyle;
use crate::impl_enum_convert_exact;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum EntityStyle {
  Bold,
  Italic,
  Strikethrough,
  Underline,
}

impl_enum_convert_exact! {
  EntityStyle <=> FontStyle;
  Bold, Italic, Strikethrough, Underline
}

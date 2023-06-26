/*
 * Copyright 2023 - Golden_Water
 *
 * SPDX-License-Identifier: MIT
 */

use crate::bot::villa::room::message::message_builder::mhy_text_component::MhyTextMsgComponent;

/// mhy text
#[derive(Debug, Default, Clone, PartialEq)]
pub struct MhyText {
  /// message content
  pub content: Vec<MhyTextMsgComponent>,
}

impl MhyText {
  /// initialize with content, quote and mentioned_info
  pub fn new(content: Vec<MhyTextMsgComponent>) -> Self {
    Self { content }
  }
}

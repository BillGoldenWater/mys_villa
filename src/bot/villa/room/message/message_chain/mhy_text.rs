/*
 * Copyright 2023 - Golden_Water
 *
 * SPDX-License-Identifier: MIT
 */

use crate::api_type::event::bot_event::bot_event_data::message_identifier::MessageIdentifier;
use crate::api_type::message::message_mhy_text::mentioned_info::MentionedInfo;
use crate::bot::villa::room::message::message_builder::mhy_text_msg_component::MhyTextMsgComponent;

/// mhy text
#[derive(Debug, Default, Clone, PartialEq)]
pub struct MhyText {
  /// message content
  pub content: Vec<MhyTextMsgComponent>,
  /// quote target
  pub quote: Option<MessageIdentifier>,
  /// members that need notify
  pub mentioned_info: Option<MentionedInfo>,
}

impl MhyText {
  /// initialize with content, quote and mentioned_info
  pub fn new(
    content: Vec<MhyTextMsgComponent>,
    quote: Option<MessageIdentifier>,
    mentioned_info: Option<MentionedInfo>,
  ) -> Self {
    Self {
      content,
      quote,
      mentioned_info,
    }
  }
}

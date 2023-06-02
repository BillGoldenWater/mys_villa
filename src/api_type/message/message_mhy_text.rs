use serde::{Deserialize, Serialize};

use crate::api_type::message::message_mhy_text::mentioned_info::MentionedInfo;
use crate::api_type::message::message_mhy_text::msg_content::MsgContent;
use crate::api_type::message::message_mhy_text::quote_info::QuoteInfo;

/// definition of entity data
pub mod entity_data;
/// definition of mentioned info
pub mod mentioned_info;
/// definition of msg content
pub mod msg_content;
/// definition of quote info
pub mod quote_info;
/// definition of text entity
pub mod text_entity;

/// message mhy text
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MessageMhyText {
  /// content
  pub content: MsgContent,
  /// mention info
  #[serde(default)]
  pub mentioned_info: Option<MentionedInfo>,
  /// quote info
  #[serde(default)]
  pub quote: Option<QuoteInfo>,
}

impl MessageMhyText {
  /// initialize with content, mentioned_info and quote
  pub fn new(
    content: MsgContent,
    mentioned_info: Option<MentionedInfo>,
    quote: Option<QuoteInfo>,
  ) -> Self {
    Self {
      content,
      mentioned_info,
      quote,
    }
  }
}

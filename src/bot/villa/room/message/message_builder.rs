/*
 * Copyright 2023 - Golden_Water
 *
 * SPDX-License-Identifier: MIT
 */

use crate::api_type::event::bot_event::bot_event_data::message_identifier::MessageIdentifier;
use crate::api_type::message::message_object::message_content::image::Image;
use crate::api_type::message::message_object::message_content::mhy_post::MhyPost;
use crate::api_type::message::message_object::quote_info::QuoteInfo;
use crate::api_type::message::message_object::MessageObject;
use crate::bot::villa::room::message::message_builder::content_builder::ContentBuilder;
use crate::bot::villa::room::message::message_builder::mhy_text_builder::MhyTextBuilder;

/// hub of message content builders
pub mod content_builder;
/// provide builder logic for MHY:Text
pub mod mhy_text_builder;
/// definition of mhy text component
pub mod mhy_text_component;

/// message builder
#[derive(Debug, Default, Clone)]
pub struct MessageBuilder {
  quote: Option<QuoteInfo>,

  content_builder: ContentBuilder,
}

impl MessageBuilder {
  /// convert/get builder of MHY:Text
  pub fn mhy_text<F: FnOnce(MhyTextBuilder) -> MhyTextBuilder>(mut self, f: F) -> Self {
    self.content_builder = self.content_builder.mhy_text(f);
    self
  }

  /// convert/replace MHY:Image
  pub fn mhy_image(mut self, image: Image) -> Self {
    self.content_builder = self.content_builder.mhy_image(image);
    self
  }

  /// convert/replace MHY:Post
  pub fn mhy_post(mut self, post: MhyPost) -> Self {
    self.content_builder = self.content_builder.mhy_post(post);
    self
  }

  /// append text, convert to MHY:Text if not already is
  pub fn text(self, text: impl Into<String>) -> Self {
    self.mhy_text(|m| m.text(text))
  }

  /// set quote info
  pub fn with_quote(mut self, quote_msg: impl Into<MessageIdentifier>) -> Self {
    self.quote = Some(quote_msg.into().into());
    self
  }

  /// remove quote info
  pub fn remove_quote(mut self) -> Self {
    self.quote = None;
    self
  }

  /// build [MessageObject]
  pub fn build(self) -> MessageObject {
    MessageObject {
      mentioned_info: self.content_builder.gen_mentioned_info(),
      content: self.content_builder.build(),
      quote: self.quote,
    }
  }
}

/*
 * Copyright 2023 - Golden_Water
 *
 * SPDX-License-Identifier: MIT
 */

use crate::api_type::message::message_object::MessageObject;

/// definition of builder logic for building a sub builder
pub mod message_builder_builder;
/// provide builder logic for [MessageObject::MhyText]
pub mod mhy_text_msg_builder;
/// definition of mhy text msg component
pub mod mhy_text_msg_component;

/// define build interface for a message builder
pub trait MessageBuilder {
  /// build a [MessageObject]
  fn build(self) -> MessageObject;
}

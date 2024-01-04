/*
 * Copyright 2023 - Golden_Water
 *
 * SPDX-License-Identifier: MIT
 */

use serde::Deserialize;

use crate::api_type::{
  event::bot_event::bot_event_data::{
    add_quick_emoticon_metadata::AddQuickEmoticonMetadata, audit_metadata::AuditMetadata,
    audit_status::AuditStatus, send_message_metadata::SendMessageMetadata,
  },
  message::message_object::MessageObject,
};

/// definition of add quick emoticon metadata
pub mod add_quick_emoticon_metadata;
/// definition of audit metadata
pub mod audit_metadata;
/// definition of audit status
pub mod audit_status;
/// definition of message identifier
pub mod message_identifier;
/// definition of send message metadata
pub mod send_message_metadata;

/// bot event data
#[derive(Debug, Deserialize)]
pub enum BotEventData {
  /// new member
  JoinVilla {
    /// uid
    #[serde(rename = "join_uid")]
    uid: u64,
    /// join time
    join_at: i64,
    /// nickname
    #[serde(rename = "join_user_nickname")]
    nickname: String,
  },
  /// new message
  SendMessage {
    /// metadata
    #[serde(flatten)]
    metadata: SendMessageMetadata,
    /// content
    #[serde(flatten)]
    content: MessageObject,
  },
  /// bot added to a villa
  CreateRobot {
    /// villa id
    villa_id: u64,
  },
  /// bot removed from a villa
  DeleteRobot {
    /// villa id
    villa_id: u64,
  },
  /// member reacted to a message that send by this bot
  AddQuickEmoticon {
    /// metadata
    #[serde(flatten)]
    metadata: AddQuickEmoticonMetadata,
    /// emoticon id
    emoticon_id: u64,
    /// emoticon content
    emoticon: String,
    /// is or not cancel a react
    #[serde(default)]
    is_cancel: bool,
  },
  /// audit finished
  AuditCallback {
    /// metadata
    #[serde(flatten)]
    metadata: AuditMetadata,
    /// pass through content
    pass_through: Option<String>,
    /// audit result
    audit_result: AuditStatus,
  },
}

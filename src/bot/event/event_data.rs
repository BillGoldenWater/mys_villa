/*
 * Copyright 2023 - Golden_Water
 *
 * SPDX-License-Identifier: MIT
 */

use crate::api_type::event::bot_event::bot_event_data::add_quick_emoticon_metadata::AddQuickEmoticonMetadata;
use crate::api_type::event::bot_event::bot_event_data::send_message_metadata::SendMessageMetadata;
use crate::api_type::event::bot_event::bot_event_data::BotEventData;
use crate::api_type::message::message_object::MessageObject;

/// event data
#[derive(Debug)]
pub enum EventData {
  /// new member
  JoinVilla {
    /// uid
    uid: u64,
    /// join time
    join_at: i64,
    /// nickname
    nickname: String,
  },
  /// new message
  SendMessage {
    /// metadata
    metadata: SendMessageMetadata,
    /// content
    content: MessageObject,
  },
  /// bot added to a villa
  CreateRobot,
  /// bot removed from a villa
  DeleteRobot,
  /// member reacted to a message that send by this bot
  AddQuickEmoticon {
    /// metadata
    metadata: AddQuickEmoticonMetadata,
    /// emoticon id
    emoticon_id: u64,
    /// emoticon content
    emoticon: String,
    /// is or not cancel a react
    is_cancel: bool,
  },
}

impl From<BotEventData> for EventData {
  fn from(value: BotEventData) -> Self {
    match value {
      BotEventData::JoinVilla {
        uid,
        nickname,
        join_at,
      } => Self::JoinVilla {
        uid,
        nickname,
        join_at,
      },
      BotEventData::SendMessage { metadata, content } => Self::SendMessage { metadata, content },
      BotEventData::CreateRobot { .. } => Self::CreateRobot,
      BotEventData::DeleteRobot { .. } => Self::DeleteRobot,
      BotEventData::AddQuickEmoticon {
        metadata,
        emoticon_id,
        emoticon,
        is_cancel,
      } => Self::AddQuickEmoticon {
        metadata,
        emoticon_id,
        emoticon,
        is_cancel,
      },
    }
  }
}

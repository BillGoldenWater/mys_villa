/*
 * Copyright 2023 - Golden_Water
 *
 * SPDX-License-Identifier: MIT
 */

use CommandTryFromEventError as CmdErr;

use crate::{
  api_type::{
    event::bot_event::{
      bot_command::BotCommand, bot_event_data::send_message_metadata::SendMessageMetadata,
    },
    message::message_object::MessageObject,
  },
  bot::{
    event::{event_data::EventData, Event},
    villa::room::message::{
      message_chain::MessageChain, message_chain_matcher::mhy_text_matcher::MhyTextMatcher,
    },
  },
  error::VError,
};

/// command
#[derive(Debug)]
pub struct Command {
  /// raw event
  pub event: Event,
  /// raw message metadata
  pub message_meta: SendMessageMetadata,
  /// raw text message
  pub message_mhy_text: MessageObject,

  /// command info
  pub info: BotCommand,
  /// command args
  pub args: Vec<String>,
}

/// command try from event error
#[derive(Debug, PartialEq, thiserror::Error)]
pub enum CommandTryFromEventError {
  /// event isn't send message
  #[error("event isn't send message")]
  NotSendMessage,
  /// empty message after mention
  #[error("empty message after mention")]
  EmptyMessage,
  /// message isn't a command
  #[error("")]
  NotCommand,
}

impl TryFrom<Event> for Command {
  type Error = VError;

  fn try_from(value: Event) -> Result<Self, Self::Error> {
    // region check is send message and extract data
    let (metadata, msg) = if let EventData::SendMessage { metadata, content } = &value.data {
      (metadata, content)
    } else {
      return Err(CmdErr::NotSendMessage.into());
    };
    // endregion

    let matcher = MhyTextMatcher::default()
      .mention_bot(Option::<String>::None, Some(&value.bot_info.id))
      .text(Option::<String>::None);

    let match_result = matcher.match_fuzzy(&MessageChain::try_from(msg.clone())?)?;

    // region match command and parse args
    let command_only = match_result.captures[1]
      .as_text()
      .unwrap()
      .trim_matches(' ')
      .split(' ')
      .collect::<Vec<_>>();

    if command_only.is_empty() || (command_only.len() == 1 && command_only[0].is_empty()) {
      return Err(CmdErr::EmptyMessage.into());
    }

    let command_info = value
      .bot_info
      .commands
      .iter()
      .find(|it| it.name == command_only[0]);

    if let Some(command_info) = command_info {
      Ok(Command {
        info: command_info.clone(),
        args: command_only[1..].iter().map(|it| it.to_string()).collect(),

        message_meta: metadata.clone(),
        message_mhy_text: msg.clone(),
        event: value,
      })
    } else {
      Err(CmdErr::NotCommand.into())
    }
    // endregion
  }
}

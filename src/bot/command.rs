use log::info;

use CommandTryFromEventError as CmdErr;

use crate::api_type::event::bot_event::bot_command::BotCommand;
use crate::api_type::event::bot_event::bot_event_data::send_message_metadata::SendMessageMetadata;
use crate::api_type::message::message_mhy_text::entity_data::EntityData;
use crate::api_type::message::message_mhy_text::text_entity::TextEntity;
use crate::api_type::message::message_mhy_text::MessageMhyText;
use crate::api_type::message::message_object::MessageObject;
use crate::bot::event::event_data::EventData;
use crate::bot::event::Event;

/// command
#[derive(Debug)]
pub struct Command {
  /// raw event
  pub event: Event,
  /// raw message metadata
  pub message_meta: SendMessageMetadata,
  /// raw text message
  pub message_mhy_text: MessageMhyText,

  /// command info
  pub info: BotCommand,
  /// command args
  pub args: Vec<String>,
}

/// command try from event error
#[derive(Debug)]
pub enum CommandTryFromEventError {
  /// event isn't send message
  NotSendMessage,
  /// message isn't mention this bot
  NotMentionedThisBot,
  /// mention robot text entity offset and length incorrect
  MentionEntityLengthIncorrect {
    /// entity offset
    offset: u64,
    /// entity length
    length: u64,
    /// message length
    message_length: usize,
  },
  /// empty message after mention
  EmptyMessage,
  /// message isn't a command
  NotCommand,
}

impl TryFrom<Event> for Command {
  type Error = (Event, CmdErr);

  fn try_from(value: Event) -> Result<Self, Self::Error> {
    // region check is send message and extract data
    let (metadata, msg) = if let EventData::SendMessage {
      metadata,
      content: MessageObject::MhyText(msg),
    } = &value.data
    {
      (metadata, msg)
    } else {
      return Err((value, CmdErr::NotSendMessage));
    };
    // endregion

    // region check is mention this bot and extract data
    let mention_entity = msg.content.entities.iter().find(|it| {
      if let EntityData::MentionedRobot { ref bot_id } = it.entity {
        info!("{bot_id} : {}", value.bot_info.id);
        *bot_id == value.bot_info.id
      } else {
        false
      }
    });

    let (offset, length) = if let Some(TextEntity {
      offset,
      length,
      entity: EntityData::MentionedRobot { .. },
    }) = mention_entity
    {
      (offset, length)
    } else {
      return Err((value, CmdErr::NotMentionedThisBot));
    };
    // endregion

    // region check entity length and calc command offset
    let cmd_offset = (offset + length) as usize;

    if msg.content.text.len() < cmd_offset {
      let offset = *offset;
      let length = *length;
      let message_length = msg.content.text.len();

      return Err((
        value,
        CmdErr::MentionEntityLengthIncorrect {
          offset,
          length,
          message_length,
        },
      ));
    }
    // endregion

    // region match command and parse args
    let command_only = msg.content.text[cmd_offset..]
      .trim_matches(' ')
      .split(' ')
      .collect::<Vec<_>>();

    if command_only.is_empty() || (command_only.len() == 1 && command_only[0].is_empty()) {
      return Err((value, CmdErr::EmptyMessage));
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
      Err((value, CmdErr::NotCommand))
    }
    // endregion
  }
}

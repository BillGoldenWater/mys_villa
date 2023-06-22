/*
 * Copyright 2023 - Golden_Water
 *
 * SPDX-License-Identifier: MIT
 */

use crate::api_type::event::bot_event::bot_event_data::message_identifier::MessageIdentifier;
use crate::api_type::message::message_mhy_text::entity_data::EntityData;
use crate::api_type::message::message_mhy_text::mentioned_info::MentionedInfo;
use crate::api_type::message::message_mhy_text::msg_content::MsgContent;
use crate::api_type::message::message_mhy_text::quote_info::QuoteInfo;
use crate::api_type::message::message_mhy_text::text_entity::TextEntity;
use crate::api_type::message::message_mhy_text::MessageMhyText;
use crate::api_type::message::message_object::MessageObject;
use crate::bot::bot_event_handler::BotEventHandler;
use crate::bot::villa::room::message::message_builder::mhy_text_msg_component::link::Link;
use crate::bot::villa::room::message::message_builder::mhy_text_msg_component::mention_bot::MentionBot;
use crate::bot::villa::room::message::message_builder::mhy_text_msg_component::mention_user::MentionUser;
use crate::bot::villa::room::message::message_builder::mhy_text_msg_component::villa_room_link::VillaRoomLink;
use crate::bot::villa::room::message::message_builder::mhy_text_msg_component::MhyTextMsgComponent as Component;
use crate::bot::villa::room::message::message_builder::MessageBuilder;
use crate::bot::villa::Villa;
use crate::error::VResult;
use crate::request::request_executor::RequestExecutor;
use crate::utils::unicode_utils::len_utf16;

/// builder of MHY:Text
#[derive(Debug, Clone)]
pub struct MhyTextMsgBuilder<
  'villa,
  State,
  EventHandler: BotEventHandler<State, ReqExecutor>,
  ReqExecutor: RequestExecutor,
> {
  villa: &'villa Villa<'villa, State, EventHandler, ReqExecutor>,

  components: Vec<Component>,
  spacer: Option<String>,

  quote_info: Option<QuoteInfo>,
}

impl<
    'villa,
    State,
    EventHandler: BotEventHandler<State, ReqExecutor>,
    ReqExecutor: RequestExecutor,
  > MhyTextMsgBuilder<'villa, State, EventHandler, ReqExecutor>
{
  /// initialize with villa
  pub fn new(villa: &'villa Villa<'villa, State, EventHandler, ReqExecutor>) -> Self {
    Self {
      villa,
      components: vec![],
      spacer: Some(' '.to_string()),
      quote_info: None,
    }
  }

  // region append component
  /// push a text component
  pub fn text(self, text: impl Into<String>) -> Self {
    self.push(Component::Text(text.into())).append_spacer()
  }

  /// push a mention user component
  pub fn mention_user_full(self, user_name: impl Into<String>, uid: u64) -> Self {
    self
      .push(Component::MentionUser(MentionUser::new(user_name, uid)))
      .append_spacer()
  }

  /// push a mention user component, auto fetch user name
  pub async fn mention_user_by_id(
    self,
    uid: u64,
  ) -> VResult<MhyTextMsgBuilder<'villa, State, EventHandler, ReqExecutor>> {
    self
      .villa
      .member(uid)
      .get_data()
      .await
      .map(|data| self.mention_user_full(data.info.nickname, uid))
  }

  /// push a mention all component
  pub fn mention_all(self) -> Self {
    self.push(Component::MentionAll).append_spacer()
  }

  /// push a mention bot component
  pub fn mention_bot(self, bot_name: impl Into<String>, bot_id: impl Into<String>) -> Self {
    self
      .push(Component::MentionBot(MentionBot::new(bot_name, bot_id)))
      .append_spacer()
  }

  /// push a villa room link component
  pub fn room_link_full(self, room_name: impl Into<String>, villa_id: u64, room_id: u64) -> Self {
    self
      .push(Component::VillaRoomLink(VillaRoomLink::new(
        room_name, villa_id, room_id,
      )))
      .append_spacer()
  }

  /// push a villa room link component with name and id, auto fill villa_id
  pub fn room_link(self, room_name: impl Into<String>, room_id: u64) -> Self {
    let villa_id = self.villa.id();
    self.room_link_full(room_name, villa_id, room_id)
  }

  /// push a villa room link component by room_id, auto fetch room name
  pub async fn room_link_by_id(
    self,
    room_id: u64,
  ) -> VResult<MhyTextMsgBuilder<'villa, State, EventHandler, ReqExecutor>> {
    self
      .villa
      .room(room_id)
      .get_data()
      .await
      .map(|data| self.room_link(data.info.name, room_id))
  }

  /// push a link component
  pub fn link_full(self, link_name: impl Into<String>, url: impl Into<String>) -> Self {
    self
      .push(Component::Link(Link::new(link_name, url)))
      .append_spacer()
  }

  /// push a link component, auto fill link name using url
  pub fn link(self, url: impl Into<String>) -> Self {
    let url = url.into();
    self.link_full(url.clone(), url)
  }

  /// set current spacer for spacing future components
  pub fn with_spacer(mut self, spacer: impl Into<String>) -> Self {
    self.spacer = Some(spacer.into());
    self
  }

  /// reset spacer to ' '
  pub fn reset_spacer(mut self) -> Self {
    self.spacer = Some(' '.to_string());
    self
  }

  /// remove last spacer
  pub fn trim_last_spacer(mut self) -> Self {
    if let Some(Component::Spacer(_)) = self.components.last() {
      self.components.pop();
    }
    self
  }

  /// remove current spacer
  pub fn remove_spacer(mut self) -> Self {
    self.spacer = None;
    self
  }

  /// set quote info
  pub fn with_quote(mut self, quote_msg: impl Into<MessageIdentifier>) -> Self {
    self.quote_info = Some(quote_msg.into().into());
    self
  }

  /// remove quote info
  pub fn remove_quote(mut self) -> Self {
    self.quote_info = None;
    self
  }

  fn push(mut self, component: Component) -> Self {
    self.components.push(component);
    self
  }

  fn append_spacer(mut self) -> Self {
    if let Some(ref spacer) = self.spacer {
      self.components.push(Component::Spacer(spacer.clone()))
    }
    self
  }
  // endregion
}

impl<
    'villa,
    State,
    EventHandler: BotEventHandler<State, ReqExecutor>,
    ReqExecutor: RequestExecutor,
  > MessageBuilder for MhyTextMsgBuilder<'villa, State, EventHandler, ReqExecutor>
{
  /// build to [MessageObject]
  fn build(mut self) -> MessageObject {
    self = self.trim_last_spacer();

    let mut mentioned_info = Option::<MentionedInfo>::None;

    let mut text_content = String::new();
    let mut entities = Vec::<TextEntity>::new();

    for component in self.components {
      match component {
        Component::Text(text) => {
          text_content.push_str(&text);
        }
        Component::Spacer(spacer) => text_content.push_str(&spacer),
        Component::MentionUser(MentionUser { uid, user_name }) => {
          let content = format!("@{user_name}");
          let uid = uid.to_string();

          mentioned_info.add_member(uid.clone());
          entities.push_entity(
            &mut text_content,
            content,
            EntityData::MentionedUser { user_id: uid },
          );
        }
        Component::MentionAll => {
          let content = "@全体成员".to_string();

          mentioned_info.upgrade_to_all();
          entities.push_entity(&mut text_content, content, EntityData::MentionedAll);
        }
        Component::MentionBot(MentionBot { bot_id, bot_name }) => {
          let content = format!("@{bot_name}");

          mentioned_info.add_member(bot_id.clone());
          entities.push_entity(
            &mut text_content,
            content,
            EntityData::MentionedRobot { bot_id },
          );
        }
        Component::VillaRoomLink(VillaRoomLink {
          villa_id,
          room_id,
          room_name,
        }) => {
          let content = format!("#{room_name}");

          entities.push_entity(
            &mut text_content,
            content,
            EntityData::VillaRoomLink {
              room_id: room_id.to_string(),
              villa_id: villa_id.to_string(),
            },
          )
        }
        Component::Link(Link { link_name, url }) => {
          entities.push_entity(&mut text_content, link_name, EntityData::Link { url });
        }
      }
    }

    MessageObject::MhyText(MessageMhyText::new(
      MsgContent::new(text_content, entities),
      mentioned_info,
      self.quote_info,
    ))
  }
}

trait MentionedInfoStorage {
  fn upgrade_to_all(&mut self);

  fn add_member(&mut self, id: String);
}

impl MentionedInfoStorage for Option<MentionedInfo> {
  fn upgrade_to_all(&mut self) {
    self.replace(MentionedInfo::All);
  }

  fn add_member(&mut self, id: String) {
    match self {
      None => {
        self.replace(MentionedInfo::Partial(vec![id]));
      }
      Some(mentioned_info) => {
        if let MentionedInfo::Partial(members) = mentioned_info {
          // if current is partial, then add member, if current is all, then doesn't need add member
          members.push(id)
        }
      }
    }
  }
}

trait TextEntityStorage {
  fn push_entity(&mut self, previous_content: &mut String, content: String, entity: EntityData);
}

impl TextEntityStorage for Vec<TextEntity> {
  fn push_entity(&mut self, previous_content: &mut String, content: String, entity: EntityData) {
    self.push(TextEntity::new(
      len_utf16(&previous_content) as u64,
      len_utf16(&content) as u64,
      entity,
    ));

    previous_content.push_str(&content)
  }
}

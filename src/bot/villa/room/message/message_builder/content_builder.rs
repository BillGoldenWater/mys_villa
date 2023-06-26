/*
 * Copyright 2023 - Golden_Water
 *
 * SPDX-License-Identifier: MIT
 */

use crate::api_type::message::message_object::mentioned_info::MentionedInfo;
use crate::api_type::message::message_object::message_content::MessageContent;
use crate::bot::bot_event_handler::BotEventHandler;
use crate::bot::villa::room::message::message_builder::mhy_text_builder::MhyTextBuilder;
use crate::bot::villa::Villa;
use crate::request::request_executor::RequestExecutor;

/// define build interface for a message content builder
pub trait ContentBuilder {
  /// build [MessageContent]
  fn build(self) -> MessageContent;
  /// generate [MentionedInfo]
  fn gen_mentioned_info(&self) -> Option<MentionedInfo>;
}

/// message content builders hub
#[derive(Debug, Clone)]
pub enum ContentBuilderHub<
  'villa,
  State,
  EventHandler: BotEventHandler<State, ReqExecutor>,
  ReqExecutor: RequestExecutor,
> {
  /// MHY:Text
  MhyText(MhyTextBuilder<'villa, State, EventHandler, ReqExecutor>),
}

impl<
    'villa,
    State,
    EventHandler: BotEventHandler<State, ReqExecutor>,
    ReqExecutor: RequestExecutor,
  > ContentBuilderHub<'villa, State, EventHandler, ReqExecutor>
{
  /// initialize with villa
  pub fn new(villa: &'villa Villa<'villa, State, EventHandler, ReqExecutor>) -> Self {
    Self::MhyText(MhyTextBuilder::new(villa))
  }

  /// convert/get MhyTextBuilder
  pub fn mhy_text<
    F: FnOnce(
      MhyTextBuilder<'villa, State, EventHandler, ReqExecutor>,
    ) -> MhyTextBuilder<'villa, State, EventHandler, ReqExecutor>,
  >(
    self,
    villa: &'villa Villa<'villa, State, EventHandler, ReqExecutor>,
    f: F,
  ) -> Self {
    let result = match self {
      ContentBuilderHub::MhyText(mhy_text_builder) => f(mhy_text_builder),
      _ => f(MhyTextBuilder::new(villa)),
    };

    Self::MhyText(result)
  }
}

impl<
    'villa,
    State,
    EventHandler: BotEventHandler<State, ReqExecutor>,
    ReqExecutor: RequestExecutor,
  > ContentBuilder for ContentBuilderHub<'villa, State, EventHandler, ReqExecutor>
{
  fn build(self) -> MessageContent {
    match self {
      ContentBuilderHub::MhyText(mhy_text) => mhy_text.build(),
    }
  }

  fn gen_mentioned_info(&self) -> Option<MentionedInfo> {
    match self {
      ContentBuilderHub::MhyText(mhy_text) => mhy_text.gen_mentioned_info(),
    }
  }
}

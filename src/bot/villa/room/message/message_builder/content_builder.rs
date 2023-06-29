/*
 * Copyright 2023 - Golden_Water
 *
 * SPDX-License-Identifier: MIT
 */

use crate::api_type::message::message_object::mentioned_info::MentionedInfo;
use crate::api_type::message::message_object::message_content::image::Image;
use crate::api_type::message::message_object::message_content::mhy_post::MhyPost;
use crate::api_type::message::message_object::message_content::MessageContent;
use crate::bot::bot_event_handler::BotEventHandler;
use crate::bot::villa::room::message::message_builder::mhy_text_builder::MhyTextBuilder;
use crate::bot::villa::Villa;
use crate::request::request_executor::RequestExecutor;

/// message content builders hub
#[derive(Debug, Clone)]
pub enum ContentBuilder<
  'villa,
  State,
  EventHandler: BotEventHandler<State, ReqExecutor>,
  ReqExecutor: RequestExecutor,
> {
  /// MHY:Text
  MhyText(MhyTextBuilder<'villa, State, EventHandler, ReqExecutor>),
  /// MHY:Image
  MhyImage(Image),
  /// MHY:Post
  MhyPost(MhyPost),
}

impl<
    'villa,
    State,
    EventHandler: BotEventHandler<State, ReqExecutor>,
    ReqExecutor: RequestExecutor,
  > ContentBuilder<'villa, State, EventHandler, ReqExecutor>
{
  /// initialize with villa
  pub fn new(villa: &'villa Villa<'villa, State, EventHandler, ReqExecutor>) -> Self {
    Self::MhyText(MhyTextBuilder::new(villa))
  }

  /// convert/get [MhyTextBuilder]
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
      Self::MhyText(mhy_text_builder) => f(mhy_text_builder),
      _ => f(MhyTextBuilder::new(villa)),
    };

    Self::MhyText(result)
  }

  /// convert/replace MHY:Image
  pub fn mhy_image(self, image: Image) -> Self {
    Self::MhyImage(image)
  }

  /// convert/replace MHY:Post
  pub fn mhy_post(self, post: MhyPost) -> Self {
    Self::MhyPost(post)
  }

  /// build to [MessageContent]
  pub fn build(self) -> MessageContent {
    match self {
      ContentBuilder::MhyText(mhy_text) => mhy_text.build(),
      ContentBuilder::MhyImage(image) => MessageContent::MhyImage(image),
      ContentBuilder::MhyPost(mhy_post) => MessageContent::MhyPost(mhy_post),
    }
  }

  /// generate mentioned info
  pub fn gen_mentioned_info(&self) -> Option<MentionedInfo> {
    match self {
      ContentBuilder::MhyText(mhy_text) => mhy_text.gen_mentioned_info(),
      ContentBuilder::MhyImage(_) | ContentBuilder::MhyPost(_) => None,
    }
  }
}

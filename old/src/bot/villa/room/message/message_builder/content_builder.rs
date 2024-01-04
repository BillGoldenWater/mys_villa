/*
 * Copyright 2023 - Golden_Water
 *
 * SPDX-License-Identifier: MIT
 */

use crate::{
  api_type::message::message_object::{
    mentioned_info::MentionedInfo,
    message_content::{image::Image, mhy_post::MhyPost, MessageContent},
  },
  bot::{
    bot_event_handler::BotEventHandler,
    villa::{room::message::message_builder::mhy_text_builder::MhyTextBuilder, Villa},
  },
  error::VResult,
  request::request_executor::RequestExecutor,
};

/// message content builders hub
#[derive(Debug, Clone)]
pub enum ContentBuilder {
  /// MHY:Text
  MhyText(MhyTextBuilder),
  /// MHY:Image
  MhyImage(Image),
  /// MHY:Post
  MhyPost(MhyPost),
}

impl ContentBuilder {
  /// convert/get [MhyTextBuilder]
  pub fn mhy_text<F: FnOnce(MhyTextBuilder) -> MhyTextBuilder>(self, f: F) -> Self {
    let result = match self {
      Self::MhyText(mhy_text_builder) => f(mhy_text_builder),
      _ => f(MhyTextBuilder::default()),
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

  /// transfer attached image
  pub async fn transfer_image<
    State: Sync,
    EventHandler: BotEventHandler<State, ReqExecutor>,
    ReqExecutor: RequestExecutor + Sync,
  >(
    &mut self,
    villa: &Villa<'_, State, EventHandler, ReqExecutor>,
  ) -> VResult<()> {
    match self {
      ContentBuilder::MhyText(text) => text.transfer_image(villa).await?,
      ContentBuilder::MhyImage(image) => {
        image.url = villa.transfer_image(&image.url).await?;
      }
      ContentBuilder::MhyPost(_) => {}
    }
    Ok(())
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

impl Default for ContentBuilder {
  fn default() -> Self {
    Self::MhyText(MhyTextBuilder::default())
  }
}

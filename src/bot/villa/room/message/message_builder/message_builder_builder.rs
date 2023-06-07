/*
 * Copyright 2023 - Golden_Water
 *
 * SPDX-License-Identifier: MIT
 */

use crate::bot::bot_event_handler::BotEventHandler;
use crate::bot::villa::room::message::message_builder::mhy_text_msg_builder::MhyTextMsgBuilder;
use crate::bot::villa::Villa;
use crate::request::request_executor::RequestExecutor;

/// message builder builder
#[derive(Debug)]
pub struct MessageBuilderBuilder<
  'villa,
  State,
  EventHandler: BotEventHandler<State, ReqExecutor>,
  ReqExecutor: RequestExecutor,
> {
  villa: &'villa Villa<'villa, State, EventHandler, ReqExecutor>,
}

impl<
    'villa,
    State,
    EventHandler: BotEventHandler<State, ReqExecutor>,
    ReqExecutor: RequestExecutor,
  > MessageBuilderBuilder<'villa, State, EventHandler, ReqExecutor>
{
  /// initialize with villa
  pub fn new(villa: &'villa Villa<'villa, State, EventHandler, ReqExecutor>) -> Self {
    Self { villa }
  }

  /// create a builder for MHY:Text
  pub fn mhy_text(self) -> MhyTextMsgBuilder<'villa, State, EventHandler, ReqExecutor> {
    MhyTextMsgBuilder::new(self.villa)
  }
}

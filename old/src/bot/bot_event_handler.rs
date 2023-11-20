/*
 * Copyright 2023 - Golden_Water
 *
 * SPDX-License-Identifier: MIT
 */

use std::future::Future;
use std::pin::Pin;

use crate::bot::command::Command;
use crate::bot::event::Event;
use crate::bot::villa::Villa;
use crate::bot::Bot;
use crate::error::VResult;
use crate::request::request_executor::RequestExecutor;

/// definition of bot event handler, includes:
/// - [BotEventHandler::handle] handle the event, default ignore
/// - [BotEventHandler::handle_command] for handle command call, default forwards raw event to [BotEventHandler::handle]
pub trait BotEventHandler<State: Sync, ReqExecutor: RequestExecutor + Sync> {
  /// handle the event
  fn handle<'params, 'fut>(
    &'params self,
    bot: &'params Bot<State, Self, ReqExecutor>,
    villa: Villa<'params, State, Self, ReqExecutor>,
    event: Event,
  ) -> Pin<Box<dyn Future<Output = VResult<()>> + 'fut + Send>>
  where
    Self: Sized + Sync,
    State: 'params,
    'params: 'fut,
  {
    Box::pin(async move {
      #[allow(unused_variables)]
      let (bot, villa, event) = (bot, villa, event);
      Ok(())
    })
  }

  /// handle command
  fn handle_command<'params, 'fut>(
    &'params self,
    bot: &'params Bot<State, Self, ReqExecutor>,
    villa: Villa<'params, State, Self, ReqExecutor>,
    command: Command,
  ) -> Pin<Box<dyn Future<Output = VResult<()>> + 'fut + Send>>
  where
    Self: Sized + Sync,
    State: 'params,
    'params: 'fut,
  {
    self.handle(bot, villa, command.event)
  }
}

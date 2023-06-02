use std::future::Future;
use std::pin::Pin;

use crate::bot::command::Command;
use crate::bot::event::Event;
use crate::bot::villa::Villa;
use crate::bot::Bot;
use crate::error::VResult;
use crate::request::request_executor::RequestExecutor;

/// definition of bot event handler
pub trait BotEventHandler<State, ReqExecutor: RequestExecutor> {
  /// handle the event
  fn handle<'params, 'fut>(
    &'params self,
    bot: &'params Bot<State, Self, ReqExecutor>,
    villa: Villa<'params, State, Self, ReqExecutor>,
    event: Event,
  ) -> Pin<Box<dyn Future<Output = VResult<()>> + 'fut>>
  where
    Self: Sized,
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
  ) -> Pin<Box<dyn Future<Output = VResult<()>> + 'fut>>
  where
    Self: Sized,
    State: 'params,
    'params: 'fut,
  {
    self.handle(bot, villa, command.event)
  }
}

use std::collections::HashMap;
use std::fmt::Debug;
use std::sync::Mutex;

use log::info;

use crate::api_type::emoticon::get_all_emoticon_response::GetAllEmoticonResponse;
use crate::api_type::emoticon::Emoticon;
use crate::api_type::event::bot_event::bot_event_extend_data::BotEventExtendData;
use crate::api_type::event::bot_event::BotEvent;
use crate::bot::bot_event_handler::BotEventHandler;
use crate::bot::bot_info::BotAuthInfo;
use crate::bot::bot_permission::BotPermission;
use crate::bot::command::Command;
use crate::bot::event::Event;
use crate::bot::villa::Villa;
use crate::error::VResult;
use crate::request::header_builder::HeaderBuilder;
use crate::request::request_builder::RequestBuilder;
use crate::request::request_executor::RequestExecutor;

/// definition of bot event handler
pub mod bot_event_handler;
/// information
pub mod bot_info;
/// permission
pub mod bot_permission;
/// command definition and parse logic
pub mod command;
/// event definition
pub mod event;
/// villa related logic
pub mod villa;

/// bot instance, provide api access
#[derive(Debug)]
pub struct Bot<
  State,
  EventHandler: BotEventHandler<State, ReqExecutor>,
  ReqExecutor: RequestExecutor,
> {
  info: BotAuthInfo,
  permission: Vec<BotPermission>,
  request_executor: ReqExecutor,

  // mutex for interior mutability
  state: Mutex<State>,
  event_handler: EventHandler,

  default_req_builder: RequestBuilder,
}

impl<
    State: Debug,
    EventHandler: BotEventHandler<State, ReqExecutor> + Debug,
    ReqExecutor: RequestExecutor + Debug,
  > Bot<State, EventHandler, ReqExecutor>
{
  /// initialize bot with authentication info and permission info
  pub fn new(
    info: BotAuthInfo,
    permission: Vec<BotPermission>,
    request_executor: ReqExecutor,
    shared_state: State,
    event_handler: EventHandler,
  ) -> Self {
    info!(
      "initializing bot instance with {:?}, permissions: {:?}, state: {:?}",
      info, permission, shared_state
    );

    Self {
      default_req_builder: RequestBuilder::new(HeaderBuilder::default()),
      info,
      permission,
      request_executor,
      state: Mutex::new(shared_state),
      event_handler,
    }
  }

  /// set state
  pub fn set_state(&self, new_state: State) {
    *self.state.lock().unwrap() = new_state;
  }

  /// get state
  pub fn get_state(&self) -> &Mutex<State> {
    &self.state
  }

  /// on event
  pub async fn on_event(&self, event: BotEvent) -> VResult<()> {
    let villa_id = event.robot.villa_id;
    let event = Event {
      bot_info: event.robot.template,
      metadata: event.metadata,
      data: match event.extend_data {
        BotEventExtendData::EventData(data) => data.into(),
      },
    };

    match Command::try_from(event) {
      Ok(command) => {
        self
          .event_handler
          .handle_command(self, self.villa(villa_id), command)
          .await
      }
      Err((event, _)) => {
        self
          .event_handler
          .handle(self, self.villa(villa_id), event)
          .await
      }
    }
  }

  /// create a villa instance with villa_id
  pub fn villa(&self, villa_id: u64) -> Villa<'_, State, EventHandler, ReqExecutor> {
    Villa::new(self, villa_id)
  }

  /// get all emoticon information
  pub async fn get_all_emoticons(&self) -> VResult<HashMap<u64, Emoticon>> {
    self
      .default_req_builder
      .build_get("/vila/api/bot/platform/getAllEmoticons")
      .execute_result::<GetAllEmoticonResponse, _>(&self.request_executor)
      .await
      .map(|it| it.list.into_iter().map(|it| (it.emoticon_id, it)).collect())
  }
}

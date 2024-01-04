/*
 * Copyright 2023 - Golden_Water
 *
 * SPDX-License-Identifier: MIT
 */

use std::{collections::HashMap, fmt::Debug};

use base64::Engine;
use log::info;

use crate::{
  api_type::{
    emoticon::{get_all_emoticon_response::GetAllEmoticonResponse, Emoticon},
    event::{
      bot_event::{bot_event_extend_data::BotEventExtendData, BotEvent},
      event_callback_request::EventCallbackRequest,
    },
  },
  bot::{
    bot_event_handler::BotEventHandler, bot_info::BotAuthInfo, bot_permission::BotPermission,
    command::Command, event::Event, villa::Villa,
  },
  error::VResult,
  request::{
    header_builder::HeaderBuilder, request_builder::RequestBuilder,
    request_executor::RequestExecutor,
  },
};

/// definition of bot event handler
pub mod bot_event_handler;
/// bot information, currently only authentication information
pub mod bot_info;
/// permission definition, for pre-check permission before send request, which will faster
pub mod bot_permission;
/// command definition and parse logic
pub mod command;
/// event definition
pub mod event;
/// villa related logic, includes role, group, room and message related operation
pub mod villa;

/// for execute api under bot context
/// - [Bot::get_all_emoticons] for get all [Emoticon] info
/// - [Bot::villa] create villa instance by id
#[derive(Debug)]
pub struct Bot<
  State: Sync,
  EventHandler: BotEventHandler<State, ReqExecutor>,
  ReqExecutor: RequestExecutor + Sync,
> {
  auth_info: BotAuthInfo,
  permission: Vec<BotPermission>,
  request_executor: ReqExecutor,

  state: State,
  event_handler: EventHandler,

  default_req_builder: RequestBuilder,
}

impl<
    State: Debug + Sync,
    EventHandler: BotEventHandler<State, ReqExecutor> + Debug + Sync,
    ReqExecutor: RequestExecutor + Debug + Sync,
  > Bot<State, EventHandler, ReqExecutor>
{
  /// initialize bot with authentication info and permission info
  pub fn new(
    auth_info: BotAuthInfo,
    permission: Vec<BotPermission>,
    request_executor: ReqExecutor,
    shared_state: State,
    event_handler: EventHandler,
  ) -> Self {
    info!(
      "initializing bot instance with {:?}, permissions: {:?}, state: {:?}",
      auth_info, permission, shared_state
    );

    Self {
      default_req_builder: RequestBuilder::new(HeaderBuilder::default()),
      auth_info,
      permission,
      request_executor,
      state: shared_state,
      event_handler,
    }
  }

  /// get state
  pub fn get_state(&self) -> &State {
    &self.state
  }

  /// process the raw body from callback
  pub async fn on_callback(
    &self,
    body: &[u8],
    sign_base64: Option<impl AsRef<str>>,
  ) -> VResult<()> {
    if let Some(sign) = sign_base64 {
      let sign = base64::engine::general_purpose::STANDARD.decode(sign.as_ref())?;

      self
        .auth_info
        .verify_callback_sign(String::from_utf8_lossy(body), &sign)?;
    }

    let req = serde_json::from_slice::<EventCallbackRequest>(body)?;

    self.on_event(req.event).await
  }

  /// process the deserialized event
  pub async fn on_event(&self, event: BotEvent) -> VResult<()> {
    let villa_id = event.robot.villa_id;
    let event = Event {
      bot_info: event.robot.template,
      metadata: event.metadata,
      data: match event.extend_data {
        BotEventExtendData::EventData(data) => data.into(),
      },
    };

    match Command::try_from(event.clone()) {
      Ok(command) => {
        self
          .event_handler
          .handle_command(self, self.villa(villa_id), command)
          .await
      }
      Err(_) => {
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

/// provide a simple way to initialize bot instance for testing
#[cfg(feature = "request_executor_impl")]
pub mod default {
  use crate::{
    bot::{
      bot_event_handler::BotEventHandler, bot_info::BotAuthInfo, bot_permission::BotPermission, Bot,
    },
    request::request_executor::request_executor_impl::RequestExecutorImpl,
  };

  /// default state
  #[derive(Debug)]
  pub struct State;

  /// default event handler
  #[derive(Debug)]
  pub struct EventHandler;

  impl BotEventHandler<State, RequestExecutorImpl> for EventHandler {}

  /// initialize with all default
  pub fn default() -> Bot<State, EventHandler, RequestExecutorImpl> {
    Bot::new(
      BotAuthInfo::new("", "", ""),
      BotPermission::all(),
      RequestExecutorImpl::new().expect("failed to initialize default request executor"),
      State,
      EventHandler,
    )
  }
}

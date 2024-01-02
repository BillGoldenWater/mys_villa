/*
 * Copyright 2023 - Golden_Water
 *
 * SPDX-License-Identifier: MIT
 */

use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use std::sync::Arc;

use itertools::Itertools;
use serde::de::DeserializeOwned;
use tracing::instrument;

use crate::api::villa_bot_api::get_all_emoticons::GetAllEmoticonsResponse;
use crate::api::villa_bot_api::parse_villa_res;
use crate::bot::bot_auth_info::BotAuthInfo;
use crate::bot::bot_permission::BotPermission;
use crate::bot::emoticon::Emoticon;
use crate::bot::villa::Villa;
use crate::error::VResult;
use crate::http::request::Request;
use crate::http::request_executor::RequestExecutor;

pub mod bot_auth_info;
pub mod bot_builder;
pub mod bot_permission;
pub mod emoticon;
pub mod villa;

#[derive(Clone)]
pub struct Bot {
  api_endpoint: Arc<str>,

  auth_info: Arc<BotAuthInfo>,
  permissions: Arc<[BotPermission]>,
  request_executor: Arc<dyn RequestExecutor + Send + Sync>,

  states: Arc<HashMap<TypeId, Box<dyn Any + Send + Sync>>>,
}

impl Bot {
  #[instrument(level = "debug", skip(states, request_executor))]
  pub fn new(
    api_endpoint: Arc<str>,
    auth_info: Arc<BotAuthInfo>,
    permissions: Arc<[BotPermission]>,
    request_executor: Arc<dyn RequestExecutor + Send + Sync>,
    states: Arc<HashMap<TypeId, Box<dyn Any + Send + Sync>>>,
  ) -> Self {
    Self {
      api_endpoint,
      auth_info,
      permissions,
      request_executor,
      states,
    }
  }

  #[instrument(level = "trace", skip_all)]
  pub fn get_state<T: Any + Send + Sync>(&self) -> Option<&T> {
    self
      .states
      .get(&TypeId::of::<T>())
      .and_then(|it| (*it).downcast_ref())
  }

  #[instrument(level = "trace", skip_all, fields(method = format ! ("{:?}", request.method), url = request.url))]
  pub async fn execute_bot_req<T: DeserializeOwned>(&self, request: Request) -> VResult<T> {
    let response = self
      .request_executor
      .execute(request.prepend_bot_endpoint(self.api_endpoint.as_ref()))
      .await?;
    parse_villa_res(response).map_err(Into::into)
  }

  pub async fn execute_bot_req_authed<T: DeserializeOwned>(&self, request: Request) -> VResult<T> {
    self
      .execute_bot_req(request.with_auth(&self.auth_info))
      .await
  }

  pub fn villa(&self, id: u64) -> Villa {
    Villa::new(Arc::new(self.clone()), id)
  }
}

/// api
impl Bot {
  #[instrument(level = "debug", skip_all)]
  pub async fn get_all_emoticons(&self) -> VResult<Vec<Emoticon>> {
    self
      .execute_bot_req_authed::<GetAllEmoticonsResponse>(Request::get(
        "/vila/api/bot/platform/getAllEmoticons",
      ))
      .await
      .map(|it| it.list.into_iter().map(Into::into).collect_vec())
  }
}

impl Debug for Bot {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("Bot")
      .field("auth_info", &self.auth_info)
      .field("permissions", &self.permissions)
      .finish()
  }
}

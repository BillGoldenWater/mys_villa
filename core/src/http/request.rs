/*
 * Copyright 2023 - Golden_Water
 *
 * SPDX-License-Identifier: MIT
 */

use serde::Serialize;

use crate::api::api_error::ApiResult;
use crate::bot::bot_auth_info::BotAuthInfo;
use crate::http::header_map::HeaderMap;
use crate::http::request::body::multipart::Multipart;
use crate::http::request::body::Body;
use crate::http::request::method::Method;

pub mod body;
pub mod method;

#[derive(Debug, Clone)]
pub struct Request {
  pub method: Method,
  pub url: String,
  pub headers: HeaderMap,
  pub body: Body,
}

impl Request {
  pub fn new(method: Method, url: impl Into<String>) -> Self {
    Self {
      method,
      url: url.into(),
      headers: HeaderMap::default(),
      body: Body::Empty,
    }
  }

  pub fn get(url: impl Into<String>) -> Self {
    Self::new(Method::Get, url)
  }

  pub fn post(url: impl Into<String>) -> Self {
    Self::new(Method::Post, url)
  }

  pub fn prepend_bot_endpoint(mut self, endpoint: &str) -> Self {
    self.url = format!("{endpoint}{path}", path = self.url);
    self
  }
}

/// header
impl Request {
  pub fn with_auth(mut self, bot_auth_info: &BotAuthInfo) -> Self {
    self
      .headers
      .insert("x-rpc-bot_id".into(), bot_auth_info.id().into());
    self.headers.insert(
      "x-rpc-bot_secret".into(),
      bot_auth_info.secret_hmac().into(),
    );
    self
  }

  pub fn with_villa_id(mut self, villa_id: u64) -> Self {
    self
      .headers
      .insert("x-rpc-bot_villa_id".into(), villa_id.to_string());
    self
  }
}

/// body
impl Request {
  pub fn with_body(mut self, body: Body) -> Self {
    self.body = body;
    self
  }

  pub fn with_json(self, json: impl Serialize) -> ApiResult<Self> {
    Ok(self.with_body(Body::Json(serde_json::to_string(&json)?)))
  }

  pub fn with_multipart(self, multipart: Multipart) -> Self {
    self.with_body(Body::Multipart(multipart))
  }
}

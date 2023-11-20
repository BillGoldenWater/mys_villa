/*
 * Copyright 2023 - Golden_Water
 *
 * SPDX-License-Identifier: MIT
 */

use serde::Serialize;
use serde_json::{json, Value};

use crate::request::header_builder::HeaderBuilder;
use crate::request::method::Method;
use crate::request::Request;

/// reusable request builder
#[derive(Debug)]
pub struct RequestBuilder {
  header: HeaderBuilder,
}

impl RequestBuilder {
  /// initialize a request builder with header
  pub fn new(header: HeaderBuilder) -> Self {
    Self { header }
  }

  /// build a request with method, path and body
  pub fn build_with_body<UrlPath, Body>(
    &self,
    method: Method,
    path: UrlPath,
    body: Body,
  ) -> Request<'_, Body>
  where
    UrlPath: Into<String>,
    Body: Serialize + Send + Sync,
  {
    Request {
      method,
      path: path.into(),
      header: &self.header,
      body,
    }
  }

  /// build a get request with path
  pub fn build_get<UrlPath>(&self, path: UrlPath) -> Request<'_, Value>
  where
    UrlPath: Into<String>,
  {
    self.build_with_body(Method::GET, path, json! {{  }})
  }

  /// build a get request with path and body
  pub fn build_get_with_body<UrlPath, Body>(&self, path: UrlPath, body: Body) -> Request<'_, Body>
  where
    UrlPath: Into<String>,
    Body: Serialize + Send + Sync,
  {
    self.build_with_body(Method::GET, path, body)
  }

  /// build a post request with path and body
  pub fn build_post_with_body<UrlPath, Body>(&self, path: UrlPath, body: Body) -> Request<'_, Body>
  where
    UrlPath: Into<String>,
    Body: Serialize + Send + Sync,
  {
    self.build_with_body(Method::POST, path, body)
  }
}

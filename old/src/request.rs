/*
 * Copyright 2023 - Golden_Water
 *
 * SPDX-License-Identifier: MIT
 */

use serde::{de::DeserializeOwned, Serialize};
use serde_json::Value;

use crate::{
  error::VResult,
  request::{header_builder::HeaderBuilder, method::Method, request_executor::RequestExecutor},
  response::Response,
};

/// provide ability to build a header easily
pub mod header_builder;
/// define http request method
pub mod method;
/// provide ability to build a request with only un-reusable data
pub mod request_builder;
/// provide ability to send request
pub mod request_executor;

/// request struct
#[derive(Debug)]
pub struct Request<'header, Body: Serialize + Send + Sync> {
  /// the method using in http
  pub method: Method,
  /// api path
  pub path: String,
  /// header builder
  pub header: &'header HeaderBuilder,
  body: Body,
}

impl<Body: Serialize + Send + Sync> Request<'_, Body> {
  /// serialize the body to json string
  pub fn body_to_string(&self) -> VResult<String> {
    Ok(serde_json::to_string(&self.body)?)
  }

  /// execute by executor
  pub async fn execute<ResBody, ReqExecutor>(
    self,
    executor: &ReqExecutor,
  ) -> VResult<Response<ResBody>>
  where
    ResBody: DeserializeOwned + Sync,
    ReqExecutor: RequestExecutor + Sync,
  {
    executor.execute(self).await
  }

  /// execute and parse response into result
  pub async fn execute_result<ResBody, ReqExecutor>(
    self,
    executor: &ReqExecutor,
  ) -> VResult<ResBody>
  where
    ResBody: DeserializeOwned + Sync,
    ReqExecutor: RequestExecutor + Sync,
  {
    self.execute(executor).await?.to_result()
  }

  /// execute and ignore return value
  pub async fn execute_no_return<ReqExecutor>(self, executor: &ReqExecutor) -> VResult<()>
  where
    ReqExecutor: RequestExecutor + Sync,
  {
    self
      .execute::<Value, _>(executor)
      .await?
      .to_result()
      .map(|_| ())
  }
}

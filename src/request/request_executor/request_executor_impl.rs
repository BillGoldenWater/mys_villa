/*
 * Copyright 2023 - Golden_Water
 *
 * SPDX-License-Identifier: MIT
 */

use std::future::Future;
use std::pin::Pin;
use std::time::Duration;

use log::{debug, error};
use reqwest::Method as RMethod;
use reqwest::{Client, ClientBuilder};
use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::error::{VError, VResult};
use crate::request::method::Method;
use crate::request::request_executor::RequestExecutor;
use crate::request::Request;
use crate::response::Response;

/// default implementation of [RequestExecutor], using reqwest
#[derive(Debug)]
pub struct RequestExecutorImpl {
  client: Client,
}

impl RequestExecutorImpl {
  /// initialize
  pub fn new() -> VResult<Self> {
    Ok(Self {
      client: ClientBuilder::new()
        .timeout(Duration::from_secs(10))
        .build()
        .map_err(|it| VError::Other(it.to_string()))?,
    })
  }
}

impl RequestExecutor for RequestExecutorImpl {
  fn execute<'params, 'fut, ReqBody, ResBody: DeserializeOwned>(
    &'params self,
    request: Request<'params, ReqBody>,
  ) -> Pin<Box<dyn Future<Output = VResult<Response<ResBody>>> + 'fut>>
  where
    ReqBody: Serialize + Send + 'params,
    ResBody: DeserializeOwned,
    'params: 'fut,
  {
    let fut = async move {
      let body = request.body_to_string()?;
      let Request {
        method,
        header,
        path,
        ..
      } = request;

      let method = match method {
        Method::GET => RMethod::GET,
        Method::POST => RMethod::POST,
      };

      let url = format!("{base}{path}", base = Self::BASE_URL);

      debug!("{method:?} {url}");

      let mut request_builder = self.client.request(method, url);

      for (k, v) in header.iter() {
        request_builder = request_builder.header(k, v)
      }

      let response = request_builder
        .body(body)
        .send()
        .await
        .map_err(|it| VError::Other(it.to_string()))?;

      let bytes = response
        .bytes()
        .await
        .map_err(|it| VError::Other(it.to_string()))?;

      let deserialize_result = serde_json::from_slice(&bytes);

      if deserialize_result.is_err() {
        error!("{}", String::from_utf8_lossy(&bytes));
      }

      Ok(deserialize_result?)
    };

    Box::pin(fut)
  }
}

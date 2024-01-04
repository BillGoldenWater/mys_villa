/*
 * Copyright 2023 - Golden_Water
 *
 * SPDX-License-Identifier: MIT
 */

use std::{future::Future, pin::Pin};

use serde::{de::DeserializeOwned, Serialize};

use crate::{error::VResult, request::Request, response::Response};

/// default implementation of [RequestExecutor]
#[cfg(feature = "request_executor_impl")]
pub mod request_executor_impl;

/// for send http requests
pub trait RequestExecutor {
  /// api host url
  const BASE_URL: &'static str = "https://bbs-api.miyoushe.com";

  /// execute request
  fn execute<'params, 'fut, ReqBody, ResBody>(
    &'params self,
    request: Request<'params, ReqBody>,
  ) -> Pin<Box<dyn Future<Output = VResult<Response<ResBody>>> + 'fut + Send>>
  where
    Self: Sync,
    ReqBody: Serialize + Send + Sync + 'params,
    ResBody: DeserializeOwned + Sync,
    'params: 'fut;
}

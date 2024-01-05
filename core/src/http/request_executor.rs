/*
 * Copyright 2023 - Golden_Water
 *
 * SPDX-License-Identifier: MIT
 */

use std::{future::Future, pin::Pin};

use crate::{
  error::VResult,
  http::{request::Request, response::Response},
};

pub type ReqFuture<'fut> = Pin<Box<dyn Future<Output = VResult<Response>> + Send + 'fut>>;

pub trait RequestExecutor {
  fn execute<'params, 'fut>(&'params self, request: Request) -> ReqFuture<'fut>
  where
    'params: 'fut;
}

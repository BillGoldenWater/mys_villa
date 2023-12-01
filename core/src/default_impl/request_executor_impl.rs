/*
 * Copyright 2023 - Golden_Water
 *
 * SPDX-License-Identifier: MIT
 */

use std::collections::HashMap;
use std::future::Future;
use std::ops::Deref;
use std::pin::Pin;
use std::str::FromStr;
use std::time::Duration;

use reqwest::header::{self, HeaderName, HeaderValue};
use reqwest::multipart::Form;
use reqwest::{Client, ClientBuilder};

use crate::error::{VError, VResult};
use crate::http::header_map::HeaderMap;
use crate::http::request::body::multipart::part::Part;
use crate::http::request::body::Body;
use crate::http::request::method::Method;
use crate::http::request::Request;
use crate::http::request_executor::RequestExecutor;
use crate::http::response::status_code::StatusCode;
use crate::http::response::Response;

pub mod error;

#[derive(Debug)]
pub struct RequestExecutorImpl {
  client: Client,
}

impl RequestExecutorImpl {
  /// initialize
  pub fn new() -> VResult<Self> {
    let client = ClientBuilder::new()
      .timeout(Duration::from_secs(1000))
      .build()
      .map_err(|it| VError::RequestExecutor(Box::from(it)))?;

    Ok(Self { client })
  }
}

impl RequestExecutor for RequestExecutorImpl {
  fn execute<'params, 'fut>(
    &'params self,
    request: Request,
  ) -> Pin<Box<dyn Future<Output = VResult<Response>> + Send + 'fut>>
  where
    'params: 'fut,
  {
    Box::pin(async {
      execute(&self.client, request)
        .await
        .map_err(|err| VError::RequestExecutor(Box::from(err)))
    })
  }
}

async fn execute(client: &Client, request: Request) -> Result<Response, error::Error> {
  let req = match request.method {
    Method::Get => client.get(request.url),
    Method::Post => client.post(request.url),
  };

  let mut headers = header::HeaderMap::with_capacity(request.headers.len());
  for (k, v) in request.headers.deref() {
    headers.insert(HeaderName::from_str(k)?, HeaderValue::from_str(v)?);
  }

  let req = match request.body {
    Body::Empty => req.headers(headers),
    Body::Json(json) => {
      headers.insert("Content-Type", HeaderValue::from_str("application/json")?);
      req.headers(headers).body(json)
    }
    Body::Multipart(multipart) => {
      let mut form = Form::new();

      for (name, part) in multipart.into_iter() {
        use reqwest::multipart::Part as ReqPart;

        let part = match part {
          Part::Text(text) => ReqPart::text(text),
          Part::File {
            file_name,
            mime_str,
            content,
          } => {
            let mut part = reqwest::multipart::Part::bytes(content).file_name(file_name);

            if let Some(mime_str) = mime_str {
              part = part.mime_str(&mime_str)?;
            }

            part
          }
        };

        form = form.part(name, part);
      }

      req.headers(headers).multipart(form)
    }
  };

  let res = req.send().await?;

  let headers = res
    .headers()
    .iter()
    .map(|(k, v)| (k.to_string(), v.to_str().unwrap_or("").to_string()))
    .collect::<HashMap<_, _>>();

  Ok(Response {
    status_code: StatusCode::new(res.status().as_u16()),
    headers: HeaderMap::new(headers),
    body: res.bytes().await?.to_vec(),
  })
}

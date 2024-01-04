/*
 * Copyright 2023 - Golden_Water
 *
 * SPDX-License-Identifier: MIT
 */

use std::{
  collections::VecDeque,
  fmt::{Debug, Formatter},
  future::Future,
  pin::Pin,
  task::{Context, Poll},
};

use futures::{FutureExt, Stream};

use crate::{
  api::villa_bot_api::villa_api::get_villa_member::{
    GetVillaMemberRequest, GetVillaMemberResponse,
  },
  bot::villa::{member_info::MemberInfo, Villa},
  error::VResult,
  http::request::Request,
};

pub struct MemberStream {
  villa: Villa,
  buffer: VecDeque<MemberInfo>,
  next_offset: String,
  ended: bool,

  fut: Option<Pin<Box<dyn Future<Output = VResult<GetVillaMemberResponse>>>>>,
}

impl MemberStream {
  pub fn new(villa: Villa) -> Self {
    Self {
      villa,
      buffer: VecDeque::new(),
      next_offset: String::from(""),
      ended: false,

      fut: None,
    }
  }
}

impl Stream for MemberStream {
  type Item = VResult<MemberInfo>;

  fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
    const REQUEST_SIZE: u64 = 20;

    if self.ended {
      return Poll::Ready(None);
    }

    if let Some(member) = self.buffer.pop_front() {
      return Poll::Ready(Some(Ok(member)));
    }

    let fut = if let Some(fut) = &mut self.fut {
      fut
    } else {
      let req = Request::get("/vila/api/bot/platform/getVillaMembers").with_json(
        GetVillaMemberRequest::new(self.next_offset.clone(), REQUEST_SIZE),
      );

      let req = match req {
        Ok(req) => req,
        Err(err) => {
          return Poll::Ready(Some(Err(err.into())));
        }
      };

      let villa = self.villa.clone();
      let fut = async move {
        villa
          .execute_bot_req_with_villa::<GetVillaMemberResponse>(req)
          .await
      };
      self.fut = Some(Box::pin(fut));
      self.fut.as_mut().unwrap()
    };

    let res = match fut.poll_unpin(cx) {
      Poll::Ready(response) => {
        self.fut = None;
        match response {
          Ok(res) => res,
          Err(err) => {
            return Poll::Ready(Some(Err(err)));
          }
        }
      }
      Poll::Pending => {
        return Poll::Pending;
      }
    };

    self.next_offset = res.next_offset_str;
    self.buffer.extend(res.list.into_iter().map(Into::into));

    if let Some(member_info) = self.buffer.pop_front() {
      return Poll::Ready(Some(Ok(member_info)));
    }

    self.ended = true;
    Poll::Ready(None)
  }
}

impl Debug for MemberStream {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("MemberStream")
      .field("villa", &self.villa)
      .field("current", &self.buffer)
      .field("next_offset", &self.next_offset)
      .finish()
  }
}

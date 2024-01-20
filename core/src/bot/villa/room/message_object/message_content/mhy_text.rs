/*
 * Copyright 2023 - Golden_Water
 *
 * SPDX-License-Identifier: MIT
 */

use itertools::Itertools;

use crate::{
  api::villa_bot_api::villa_api::room_api::msg_content_info::msg_content::msg_mhy_text::{
    mhy_text_entity::{entity_data::EntityData, MhyTextEntity},
    MsgMhyText,
  },
  bot::villa::room::message_object::message_content::mhy_text::span::Span,
};

pub mod error;
pub mod span;

#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub struct MhyText {
  text: String,
  spans: Vec<Span>,
}

impl AsRef<str> for MhyText {
  fn as_ref(&self) -> &str {
    self.text.as_ref()
  }
}

impl<T: Into<String>> From<T> for MhyText {
  fn from(value: T) -> Self {
    Self {
      text: value.into(),
      spans: vec![],
    }
  }
}

impl From<MhyText> for MsgMhyText {
  fn from(value: MhyText) -> Self {
    let MhyText { text, spans } = value;

    if spans.is_empty() {
      return Self {
        text,
        entities: vec![],
      };
    }

    let ch_len = text
      .chars()
      .map(|ch| ch.encode_utf16(&mut [0; 2]).len())
      .collect_vec();

    let mut ch_loc = vec![usize::MAX; ch_len.len()];
    for idx in 0..ch_len.len() {
      if idx > 0 {
        ch_loc[idx] = ch_loc[idx - 1] + ch_len[idx - 1];
      } else {
        ch_loc[idx] = 0;
      }
    }

    let entities = spans
      .into_iter()
      .map(|span| {
        /*
        char_idx: 0 1  2 3
        utf16len: 1 2  1 1
        utf16loc: 0 1  3 4
        eg:       a [] a a
                  ^    ^
        3 - 0 + 1 = 4
        eg:       a [] a a
                    ^    ^
        4 - 1 + 1 = 4
        */

        let Span { start, end, entity } = span;
        let entity: EntityData = entity.into();

        if ch_len.is_empty() || ch_loc.is_empty() {
          MhyTextEntity::new(entity, 0, 0)
        } else {
          MhyTextEntity::new(
            entity,
            (ch_loc[end] - ch_loc[start] + ch_len[end]) as u64,
            ch_loc[start] as u64,
          )
        }
      })
      .collect_vec();

    Self { text, entities }
  }
}

impl TryFrom<MsgMhyText> for MhyText {
  type Error = error::Error;

  fn try_from(value: MsgMhyText) -> Result<Self, Self::Error> {
    todo!()
  }
}

// TODO: tests

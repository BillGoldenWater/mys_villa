/*
 * Copyright 2023 - Golden_Water
 *
 * SPDX-License-Identifier: MIT
 */

use crate::bot::villa::room::message::message_builder::mhy_text_msg_component::link::Link;
use crate::bot::villa::room::message::message_builder::mhy_text_msg_component::mention_bot::MentionBot;
use crate::bot::villa::room::message::message_builder::mhy_text_msg_component::mention_user::MentionUser;
use crate::bot::villa::room::message::message_builder::mhy_text_msg_component::villa_room_link::VillaRoomLink;
use crate::bot::villa::room::message::message_builder::mhy_text_msg_component::MhyTextMsgComponent;
use crate::bot::villa::room::message::message_chain::mhy_text::MhyText;
use crate::bot::villa::room::message::message_chain::MessageChain;
use crate::error::VResult;
use itertools::Itertools;
use std::fmt::Debug;

/// matcher for MHY:Text
#[derive(Debug, Clone, Default)]
pub struct MhyTextMatcher {
  items: Vec<MatchItem>,
}

impl MhyTextMatcher {
  /// text
  pub fn text(mut self, text: Option<impl Into<String>>) -> Self {
    self.items.push(MatchItem::Text(text.map(Into::into)));
    self
  }

  /// mention user
  pub fn mention_user(mut self, user_name: Option<impl Into<String>>, uid: Option<u64>) -> Self {
    self.items.push(MatchItem::MentionUser {
      user_name: user_name.map(Into::into),
      uid,
    });
    self
  }

  /// mention all
  pub fn mention_all(mut self) -> Self {
    self.items.push(MatchItem::MentionAll);
    self
  }

  /// mention bot
  pub fn mention_bot(
    mut self,
    bot_name: Option<impl Into<String>>,
    bot_id: Option<impl Into<String>>,
  ) -> Self {
    self.items.push(MatchItem::MentionBot {
      bot_name: bot_name.map(Into::into),
      bot_id: bot_id.map(Into::into),
    });
    self
  }

  /// room link
  pub fn villa_room_link(
    mut self,
    room_name: Option<impl Into<String>>,
    villa_id: Option<u64>,
    room_id: Option<u64>,
  ) -> Self {
    self.items.push(MatchItem::VillaRoomLink {
      room_name: room_name.map(Into::into),
      villa_id,
      room_id,
    });
    self
  }

  /// link
  pub fn link(
    mut self,
    link_name: Option<impl Into<String>>,
    url: Option<impl Into<String>>,
  ) -> Self {
    self.items.push(MatchItem::Link {
      link_name: link_name.map(Into::into),
      url: url.map(Into::into),
    });
    self
  }
}

impl MhyTextMatcher {
  /// match anywhere
  pub fn match_fuzzy(&self, message: &MessageChain) -> VResult<MatchFuzzyResult> {
    self.match_fuzzy_raw(&Self::unwrap_mhy_text(message)?.content)
  }

  /// match and extra components is not allowed
  pub fn match_exact(&self, message: &MessageChain) -> VResult<MatchExactResult> {
    self.match_exact_raw(&Self::unwrap_mhy_text(message)?.content)
  }

  /// return first match
  pub fn match_fuzzy_raw(&self, content: &[MhyTextMsgComponent]) -> VResult<MatchFuzzyResult> {
    let (match_result, index) = self.find_match(0, content)?;

    Ok(match_result.to_fuzzy(
      content[..index].to_vec(),
      content[index + self.items.len()..].to_vec(),
    ))
  }

  /// match and extra components is not allowed
  pub fn match_exact_raw(&self, content: &[MhyTextMsgComponent]) -> VResult<MatchExactResult> {
    if content.len() != self.items.len() {
      return Err(
        MhyTextMatchError::LengthNotMatch {
          expect: self.items.len(),
          actual: content.len(),
        }
        .into(),
      );
    }

    self.match_at(0, content)
  }

  fn unwrap_mhy_text(message: &MessageChain) -> VResult<&MhyText> {
    message
      .message_content
      .as_mhy_text()
      .ok_or(MhyTextMatchError::NotMhyText.into())
  }

  /// find first match and return with match index
  pub fn find_match(
    &self,
    start: usize,
    content: &[MhyTextMsgComponent],
  ) -> VResult<(MatchExactResult, usize)> {
    for idx in start..content.len() - start {
      let result = self
        .match_at(idx, content)
        .map(|match_result| (match_result, idx));
      if result.is_ok() {
        return result;
      }
    }

    Err(MhyTextMatchError::MatchNotFound.into())
  }

  /// try match at given index
  pub fn match_at(
    &self,
    index: usize,
    content: &[MhyTextMsgComponent],
  ) -> VResult<MatchExactResult> {
    let len = content.len() - index;
    if len < self.items.len() {
      return Err(MhyTextMatchError::LengthTooShort {
        expect: self.items.len(),
        actual: len,
      })?;
    }

    let captures = self
      .items
      .iter()
      .zip(&content[index..])
      .map(|(match_item, content)| match_item.match_result(content))
      .collect::<Result<Vec<_>, _>>()?
      .into_iter()
      .flatten()
      .cloned()
      .collect_vec();

    Ok(MatchExactResult::new(captures))
  }
}

/// result of exact match
#[derive(Debug, Default, Clone, PartialEq)]
pub struct MatchExactResult {
  /// captured values
  pub captures: Vec<MhyTextMsgComponent>,
}

impl MatchExactResult {
  /// initialize with captures
  pub fn new(captures: Vec<MhyTextMsgComponent>) -> Self {
    Self { captures }
  }

  /// convert to fuzzy match result
  pub fn to_fuzzy(
    self,
    prefix: Vec<MhyTextMsgComponent>,
    suffix: Vec<MhyTextMsgComponent>,
  ) -> MatchFuzzyResult {
    MatchFuzzyResult::new(prefix, self.captures, suffix)
  }
}

/// result of fuzzy match
#[derive(Debug, Default, Clone, PartialEq)]
pub struct MatchFuzzyResult {
  /// components that before match point
  pub prefix: Vec<MhyTextMsgComponent>,
  /// captured values
  pub captures: Vec<MhyTextMsgComponent>,
  /// components that after match items
  pub suffix: Vec<MhyTextMsgComponent>,
}

impl MatchFuzzyResult {
  /// initialize with prefix, captures and suffix
  pub fn new(
    prefix: Vec<MhyTextMsgComponent>,
    captures: Vec<MhyTextMsgComponent>,
    suffix: Vec<MhyTextMsgComponent>,
  ) -> Self {
    Self {
      prefix,
      captures,
      suffix,
    }
  }
}

#[derive(Debug, Clone, PartialEq)]
enum MatchItem {
  Text(Option<String>),
  MentionUser {
    user_name: Option<String>,
    uid: Option<u64>,
  },
  MentionAll,
  MentionBot {
    bot_name: Option<String>,
    bot_id: Option<String>,
  },
  VillaRoomLink {
    room_name: Option<String>,
    villa_id: Option<u64>,
    room_id: Option<u64>,
  },
  Link {
    link_name: Option<String>,
    url: Option<String>,
  },
}

impl MatchItem {
  /// try match a component
  pub fn match_result<'item>(
    &self,
    item: &'item MhyTextMsgComponent,
  ) -> VResult<Option<&'item MhyTextMsgComponent>> {
    /// Ok for match Err for failed, true for need value
    fn is_eq<T: PartialEq + Debug>(match_value: &Option<T>, value: &T) -> VResult<bool> {
      if let Some(match_value) = match_value {
        (match_value == value).then_some(false).ok_or(
          MhyTextMatchError::ValueNotMatch {
            expect: format!("{match_value:?}"),
            actual: format!("{value:?}"),
          }
          .into(),
        )
      } else {
        Ok(true)
      }
    }

    let mut capture = false;

    match &self {
      MatchItem::Text(text) => {
        let text_content = item.as_text().ok_or(MhyTextMatchError::TypeNotMatch)?;

        capture |= is_eq(text, &text_content.to_string())?;
      }
      MatchItem::MentionUser { user_name, uid } => {
        let MentionUser {
          user_name: user_name_content,
          uid: uid_content,
        } = item
          .as_mention_user()
          .ok_or(MhyTextMatchError::TypeNotMatch)?;

        capture |= is_eq(user_name, user_name_content)?;
        capture |= is_eq(uid, uid_content)?;
      }
      MatchItem::MentionAll => {
        item
          .is_mention_all()
          .then_some(())
          .ok_or(MhyTextMatchError::TypeNotMatch)?;
      }
      MatchItem::MentionBot { bot_name, bot_id } => {
        let MentionBot {
          bot_name: name,
          bot_id: id,
        } = item
          .as_mention_bot()
          .ok_or(MhyTextMatchError::TypeNotMatch)?;

        capture |= is_eq(bot_name, name)?;
        capture |= is_eq(bot_id, id)?;
      }
      MatchItem::VillaRoomLink {
        room_name,
        villa_id,
        room_id,
      } => {
        let VillaRoomLink {
          room_name: name,
          villa_id: villa_id_content,
          room_id: id,
        } = item
          .as_villa_room_link()
          .ok_or(MhyTextMatchError::TypeNotMatch)?;

        capture |= is_eq(room_name, name)?;
        capture |= is_eq(villa_id, villa_id_content)?;
        capture |= is_eq(room_id, id)?;
      }
      MatchItem::Link { link_name, url } => {
        let Link {
          link_name: name,
          url: url_content,
        } = item.as_link().ok_or(MhyTextMatchError::TypeNotMatch)?;

        capture |= is_eq(link_name, name)?;
        capture |= is_eq(url, url_content)?;
      }
    }

    Ok(if capture { Some(item) } else { None })
  }
}

/// match error of MHY:Text
#[derive(Debug, PartialEq, thiserror::Error)]
pub enum MhyTextMatchError {
  /// this message chain isn't MHY:Text
  #[error("this message chain isn't MHY:Text")]
  NotMhyText,
  /// message too short
  #[error("message length too short, expect at least: {expect}, actual: {actual}")]
  LengthTooShort {
    /// expected at least
    expect: usize,
    /// actual len
    actual: usize,
  },
  /// length not match
  #[error("message length not match, expect: {expect}, actual: {actual}")]
  LengthNotMatch {
    /// expected len
    expect: usize,
    /// actual len
    actual: usize,
  },
  /// value not match
  #[error("value not match, expect: {expect}, actual: {actual}")]
  ValueNotMatch {
    /// expected value
    expect: String,
    /// actual value
    actual: String,
  },
  /// type not match
  #[error("type not match")]
  TypeNotMatch,
  /// match not found
  #[error("match not found")]
  MatchNotFound,
}

#[cfg(test)]
mod tests {
  use crate::bot::default::default;
  use crate::bot::villa::room::message::message_builder::mhy_text_msg_component::MhyTextMsgComponent;
  use crate::bot::villa::room::message::message_builder::MessageBuilder;
  use crate::bot::villa::room::message::message_chain::MessageChain;
  use crate::bot::villa::room::message::message_chain_matcher::mhy_text_matcher::{
    MatchExactResult, MatchFuzzyResult, MhyTextMatcher,
  };

  #[test]
  fn test_empty() {
    let message: MessageChain = default()
      .villa(0)
      .room(0)
      .message_builder()
      .mhy_text()
      .build()
      .try_into()
      .unwrap();

    let match_result = MhyTextMatcher::default().match_exact(&message).unwrap();

    assert_eq!(match_result, MatchExactResult::new(vec![]));
  }

  #[test]
  fn test_basic() {
    let message: MessageChain = default()
      .villa(0)
      .room(0)
      .message_builder()
      .mhy_text()
      .mention_bot("arst", "bot_arst")
      .text("/command")
      .text("arg")
      .build()
      .try_into()
      .unwrap();

    let match_result = MhyTextMatcher::default()
      .mention_bot(Some("arst"), Some("bot_arst"))
      .text(Option::<String>::None)
      .match_exact(&message)
      .unwrap();

    assert_eq!(
      match_result,
      MatchExactResult::new(vec![MhyTextMsgComponent::Text(" /command arg".to_string())])
    );
  }

  #[test]
  fn test_fuzzy() {
    let message: MessageChain = default()
      .villa(0)
      .room(0)
      .message_builder()
      .mhy_text()
      .text("prefix")
      .mention_bot("arst", "bot_arst")
      .mention_all()
      .text("suffix")
      .build()
      .try_into()
      .unwrap();

    let match_result = MhyTextMatcher::default()
      .mention_bot(Some("arst"), Some("bot_arst"))
      .text(Some(" "))
      .mention_all()
      .match_fuzzy(&message)
      .unwrap();

    assert_eq!(
      match_result,
      MatchFuzzyResult::new(
        vec![MhyTextMsgComponent::Text("prefix ".to_string())],
        vec![],
        vec![MhyTextMsgComponent::Text(" suffix".to_string())]
      )
    );
  }
}

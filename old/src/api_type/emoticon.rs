/*
 * Copyright 2023 - Golden_Water
 *
 * SPDX-License-Identifier: MIT
 */

use serde::Deserialize;

/// response definition of get all emoticon
pub mod get_all_emoticon_response;

/// emoticon information
#[derive(Debug, Deserialize)]
pub struct Emoticon {
  /// emoticon id
  #[serde(deserialize_with = "serde_aux::prelude::deserialize_number_from_string")]
  pub emoticon_id: u64,
  /// description
  pub describe_text: String,
  /// icon image url
  #[serde(rename = "icon")]
  pub img_url: String,
}

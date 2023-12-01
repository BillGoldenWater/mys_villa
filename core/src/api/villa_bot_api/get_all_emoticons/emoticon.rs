/*
 * Copyright 2023 - Golden_Water
 *
 * SPDX-License-Identifier: MIT
 */

use serde::{Deserialize, Serialize};
use serde_aux::prelude::*;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct Emoticon {
  #[serde(deserialize_with = "deserialize_number_from_string")]
  pub emoticon_id: u64,
  pub describe_text: String,
  pub icon: String,
}

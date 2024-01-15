/*
 * Copyright 2023 - Golden_Water
 *
 * SPDX-License-Identifier: MIT
 */

use emoticon::Emoticon;
use serde::{Deserialize, Serialize};

pub mod emoticon;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetAllEmoticonsResponse {
  pub list: Vec<Emoticon>,
}

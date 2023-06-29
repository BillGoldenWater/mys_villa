/*
 * Copyright 2023 - Golden_Water
 *
 * SPDX-License-Identifier: MIT
 */

use serde::{Deserialize, Serialize};

/// mhy post
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MhyPost {
  /// post id
  pub post_id: String,
}

impl MhyPost {
  /// initialize with post_id
  pub fn new(post_id: impl Into<String>) -> Self {
    Self {
      post_id: post_id.into(),
    }
  }
}

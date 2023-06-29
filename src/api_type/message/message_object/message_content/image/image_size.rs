/*
 * Copyright 2023 - Golden_Water
 *
 * SPDX-License-Identifier: MIT
 */

use serde::{Deserialize, Serialize};

/// image size
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ImageSize {
  /// width
  pub width: u32,
  /// height
  pub height: u32,
}

impl ImageSize {
  /// initialize with width and height
  pub fn new(width: u32, height: u32) -> Self {
    Self { width, height }
  }
}

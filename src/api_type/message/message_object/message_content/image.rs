/*
 * Copyright 2023 - Golden_Water
 *
 * SPDX-License-Identifier: MIT
 */

use crate::api_type::message::message_object::message_content::image::image_size::ImageSize;
use serde::{Deserialize, Serialize};

/// definition of image size
pub mod image_size;

/// image
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Image {
  /// url
  pub url: String,
  /// size
  #[serde(default)]
  pub size: Option<ImageSize>,
  /// file size
  #[serde(default)]
  pub file_size: Option<u32>,
}

impl Image {
  /// initialize with url, size and file_size
  pub fn new(url: impl Into<String>, size: Option<ImageSize>, file_size: Option<u32>) -> Self {
    Self {
      url: url.into(),
      size,
      file_size,
    }
  }
}

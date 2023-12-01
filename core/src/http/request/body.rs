/*
 * Copyright 2023 - Golden_Water
 *
 * SPDX-License-Identifier: MIT
 */

use crate::http::request::body::multipart::Multipart;

pub mod multipart;

#[derive(Debug, Clone, Default)]
pub enum Body {
  #[default]
  Empty,
  Json(String),
  Multipart(Multipart),
}

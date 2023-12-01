/*
 * Copyright 2023 - Golden_Water
 *
 * SPDX-License-Identifier: MIT
 */

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum AuditContent {
  Text(String),
  Image(String),
}

impl AuditContent {
  pub fn new_text(content: String) -> Self {
    Self::Text(content)
  }

  pub fn new_image(url: String) -> Self {
    Self::Image(url)
  }

  pub fn get_type(&self) -> &'static str {
    match self {
      AuditContent::Text(_) => "AuditContentTypeText",
      AuditContent::Image(_) => "AuditContentTypeImage",
    }
  }

  pub fn to_content(self) -> String {
    match self {
      AuditContent::Text(content) => content,
      AuditContent::Image(url) => url,
    }
  }
}

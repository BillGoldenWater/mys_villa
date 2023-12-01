/*
 * Copyright 2023 - Golden_Water
 *
 * SPDX-License-Identifier: MIT
 */

#[derive(Debug, Clone)]
pub enum Part {
  Text(String),
  File {
    file_name: String,
    mime_str: Option<String>,
    content: Vec<u8>,
  },
}

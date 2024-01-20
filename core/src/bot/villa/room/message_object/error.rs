/*
 * Copyright 2023 - Golden_Water
 *
 * SPDX-License-Identifier: MIT
 */

#[derive(Debug, Clone, Eq, PartialEq, thiserror::Error)]
pub enum Error {
  #[error("error occurs when handle message content: {0}")]
  MessageContent(#[from] super::message_content::error::Error),
}

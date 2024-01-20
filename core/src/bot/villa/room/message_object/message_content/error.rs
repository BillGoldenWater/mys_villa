/*
 * Copyright 2023 - Golden_Water
 *
 * SPDX-License-Identifier: MIT
 */

#[derive(Debug, Clone, Eq, PartialEq, thiserror::Error)]
pub enum Error {
  #[error("error occurs when handle MHY:Text: {0}")]
  MhyText(#[from] super::mhy_text::error::Error),
}

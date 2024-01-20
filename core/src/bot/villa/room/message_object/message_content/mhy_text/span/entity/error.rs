/*
 * Copyright 2023 - Golden_Water
 *
 * SPDX-License-Identifier: MIT
 */

#[derive(Debug, Clone, Eq, PartialEq, thiserror::Error)]
pub enum Error {
  #[error("failed to parse number id: {0}")]
  IdParse(#[from] std::num::ParseIntError),
}

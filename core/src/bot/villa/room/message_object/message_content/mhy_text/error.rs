/*
 * Copyright 2023 - Golden_Water
 *
 * SPDX-License-Identifier: MIT
 */

#[derive(Debug, Clone, Eq, PartialEq, thiserror::Error)]
pub enum Error {
  #[error("error occurs when handle entity: {0}")]
  Entity(#[from] super::span::entity::error::Error),
}

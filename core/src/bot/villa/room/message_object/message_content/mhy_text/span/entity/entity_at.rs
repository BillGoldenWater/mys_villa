/*
 * Copyright 2023 - Golden_Water
 *
 * SPDX-License-Identifier: MIT
 */

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum EntityAt {
  All,
  User(u64),
  Bot(String),
}

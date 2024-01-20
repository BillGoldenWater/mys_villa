/*
 * Copyright 2023 - Golden_Water
 *
 * SPDX-License-Identifier: MIT
 */

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct LinkRoom {
  pub villa_id: u64,
  pub room_id: u64,
}

impl LinkRoom {
  pub fn new(villa_id: u64, room_id: u64) -> Self {
    Self { villa_id, room_id }
  }
}

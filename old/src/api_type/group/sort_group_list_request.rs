/*
 * Copyright 2023 - Golden_Water
 *
 * SPDX-License-Identifier: MIT
 */

use serde::Serialize;

/// sort group list request
#[derive(Debug, Serialize)]
pub struct SortGroupListRequest {
  villa_id: u64,
  group_list: Vec<u64>,
}

impl SortGroupListRequest {
  /// initialize with villa_id and group_list
  pub fn new(villa_id: u64, group_list: Vec<u64>) -> Self {
    Self {
      villa_id,
      group_list,
    }
  }
}

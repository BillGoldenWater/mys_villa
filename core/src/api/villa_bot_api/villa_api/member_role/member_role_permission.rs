/*
 * Copyright 2023 - Golden_Water
 *
 * SPDX-License-Identifier: MIT
 */

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MemberRolePermission {
  pub key: String,
  pub name: String,
  pub describe: String,
  // --- undocumented
  pub perm_scope: String,
}

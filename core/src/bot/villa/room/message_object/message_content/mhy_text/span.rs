/*
 * Copyright 2023 - Golden_Water
 *
 * SPDX-License-Identifier: MIT
 */

use crate::bot::villa::room::message_object::message_content::mhy_text::span::entity::Entity;

pub mod entity;

#[derive(Debug, Clone, Eq, PartialEq)]
pub(super) struct Span {
  pub(super) start: usize,
  pub(super) end: usize,
  pub(super) entity: Entity,
}

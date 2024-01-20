/*
 * Copyright 2023 - Golden_Water
 *
 * SPDX-License-Identifier: MIT
 */

use crate::bot::villa::room::message_object::message_content::mhy_text::span::entity::entity_link::link_extern::LinkExtern;
use crate::bot::villa::room::message_object::message_content::mhy_text::span::entity::entity_link::link_room::LinkRoom;

pub mod link_extern;
pub mod link_room;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum EntityLink {
  Room(LinkRoom),
  Extern(LinkExtern),
}

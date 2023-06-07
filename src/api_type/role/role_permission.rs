/*
 * Copyright 2023 - Golden_Water
 *
 * SPDX-License-Identifier: MIT
 */

use serde::{Deserialize, Serialize};

/// role permission
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RolePermission {
  /// allow using mention all
  MentionAll,
  /// allow recall anyone's message
  RecallMessage,
  /// allow pin message in room
  PinMessage,
  /// allow add/delete/edit role and manager role member
  ManageMemberRole,
  /// allow edit villa description/tag/join_condition and etc.
  EditVillaInfo,
  /// allow create/delete/reorder room/group
  ManageGroupAndRoom,
  /// allow mute member in villa
  VillaSilence,
  /// allow ban/kick member
  BlackOut,
  /// allow audit join request
  HandleApply,
  /// allow set visibility/chat_permission and edit information of a chat room
  ManageChatRoom,
  /// allow view villa data board
  ViewDataBoard,
  /// allow create/edit event
  ManageCustomEvent,
  /// allow operation of a live room
  LiveRoomOrder,
  /// allow create/delete spotlight collection
  ManageSpotlightCollection,
}

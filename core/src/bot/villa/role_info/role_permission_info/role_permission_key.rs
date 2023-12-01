/*
 * Copyright 2023 - Golden_Water
 *
 * SPDX-License-Identifier: MIT
 */

use crate::impl_enum_str_convert;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum RolePermissionKey {
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
  Unknown(String),
}

impl_enum_str_convert! {
  RolePermissionKey;
  "mention_all" => MentionAll,
  "recall_message" => RecallMessage,
  "pin_message" => PinMessage,
  "manage_member_role" => ManageMemberRole,
  "edit_villa_info" => EditVillaInfo,
  "manage_group_and_room" => ManageGroupAndRoom,
  "villa_silence" => VillaSilence,
  "black_out" => BlackOut,
  "handle_apply" => HandleApply,
  "manage_chat_room" => ManageChatRoom,
  "view_data_board" => ViewDataBoard,
  "manage_custom_event" => ManageCustomEvent,
  "live_room_order" => LiveRoomOrder,
  "manage_spotlight_collection" => ManageSpotlightCollection;
  _ => Unknown
}

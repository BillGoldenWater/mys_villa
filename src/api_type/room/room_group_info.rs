use serde::Deserialize;

use crate::api_type::group::group_info::GroupInfo;
use crate::api_type::room::room_info::RoomInfo;

/// room group info
#[derive(Debug, Deserialize)]
pub struct RoomGroupInfo {
  /// group info
  #[serde(flatten)]
  pub info: GroupInfo,
  /// room list
  pub room_list: Vec<RoomInfo>,
}

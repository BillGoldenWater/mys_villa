use serde::Deserialize;

use crate::api_type::role::role_info::RoleInfo;

/// get member role info response
#[derive(Debug, Deserialize)]
pub struct GetMemberRoleInfoResponse {
  /// role info
  pub role: RoleInfo,
}

use serde::Deserialize;

use crate::api_type::role::role_info::RoleInfo;

/// get villa member roles
#[derive(Debug, Deserialize)]
pub struct GetVillaMemberRoles {
  /// a list of role info
  pub list: Vec<RoleInfo>,
}

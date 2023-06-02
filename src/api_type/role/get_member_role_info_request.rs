use serde::Serialize;

/// get member role info request
#[derive(Debug, Serialize)]
pub struct GetMemberRoleInfoRequest {
  role_id: u64,
}

impl GetMemberRoleInfoRequest {
  /// initialize with role_id
  pub fn new(role_id: u64) -> Self {
    Self { role_id }
  }
}

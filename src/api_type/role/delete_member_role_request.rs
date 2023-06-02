use serde::Serialize;

/// delete member role request
#[derive(Debug, Serialize)]
pub struct DeleteMemberRoleRequest {
  id: u64,
}

impl DeleteMemberRoleRequest {
  /// initialize with id
  pub fn new(id: u64) -> Self {
    Self { id }
  }
}

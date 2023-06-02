use serde::Serialize;

/// operate member to role request
#[derive(Debug, Serialize)]
pub struct OperateMemberToRoleRequest {
  role_id: u64,
  uid: u64,
  is_add: bool,
}

impl OperateMemberToRoleRequest {
  /// initialize with role_id, user_uid and is_add
  pub fn new(role_id: u64, user_uid: u64, is_add: bool) -> Self {
    Self {
      role_id,
      uid: user_uid,
      is_add,
    }
  }
}

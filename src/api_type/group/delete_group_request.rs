use serde::Serialize;

/// delete group request
#[derive(Debug, Serialize)]
pub struct DeleteGroupRequest {
  group_id: u64,
}

impl DeleteGroupRequest {
  /// initialize with group_id
  pub fn new(group_id: u64) -> Self {
    Self { group_id }
  }
}

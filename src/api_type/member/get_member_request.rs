use serde::Serialize;

/// request of get member api
#[derive(Debug, Serialize)]
pub struct GetMemberRequest {
  /// member uid
  uid: u64,
}

impl GetMemberRequest {
  /// initialize with uid
  pub fn new(uid: u64) -> Self {
    Self { uid }
  }
}

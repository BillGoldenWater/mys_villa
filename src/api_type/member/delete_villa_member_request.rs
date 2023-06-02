use serde::Serialize;

/// delete villa member request
#[derive(Debug, Serialize)]
pub struct DeleteVillaMemberRequest {
  /// member uid
  uid: u64,
}

impl DeleteVillaMemberRequest {
  /// initialize with uid
  pub fn new(uid: u64) -> Self {
    Self { uid }
  }
}

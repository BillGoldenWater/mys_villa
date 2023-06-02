use serde::Deserialize;
use serde_aux::field_attributes::deserialize_number_from_string;

/// request definition of check bot access token of a member
pub mod check_member_bot_access_token_request;
/// response definition of check bot access token of a member
pub mod check_member_bot_access_token_response;

/// bot access info of a member
#[derive(Debug, Deserialize)]
pub struct BotAccessInfo {
  /// member uid
  #[serde(deserialize_with = "deserialize_number_from_string")]
  pub uid: u64,
  /// villa id
  #[serde(deserialize_with = "deserialize_number_from_string")]
  pub villa_id: u64,
  /// bot access token of a member
  #[serde(rename = "member_access_token")]
  pub bot_access_token: String,
  /// bot template id
  pub bot_tpl_id: String,
}

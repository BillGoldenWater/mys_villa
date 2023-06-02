use serde::Deserialize;

use crate::api_type::bot_access_info::BotAccessInfo;
use crate::api_type::member::member_data::MemberData;

/// check member bot access token response
#[derive(Debug, Deserialize)]
pub struct CheckMemberBotAccessTokenResponse {
  /// bot access info
  pub access_info: BotAccessInfo,
  /// member data
  pub member: MemberData,
}

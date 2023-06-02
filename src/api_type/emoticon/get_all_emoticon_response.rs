use serde::Deserialize;

use crate::api_type::emoticon::Emoticon;

/// get all emoticon response
#[derive(Debug, Deserialize)]
pub struct GetAllEmoticonResponse {
  /// a list of emoticon
  pub list: Vec<Emoticon>,
}

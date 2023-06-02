use std::collections::hash_map::Iter;
use std::collections::HashMap;
use std::fmt::{Debug, Formatter};

use crate::bot::bot_info::BotAuthInfo;

/// header builder
#[derive(Default, Clone)]
pub struct HeaderBuilder {
  headers: HashMap<String, String>,
}

impl HeaderBuilder {
  /// initialize from bot authentication information
  pub fn from_auth_info(auth_info: &BotAuthInfo) -> Self {
    Self {
      headers: HashMap::from([
        ("x-rpc-bot_id".to_string(), auth_info.id.clone()),
        ("x-rpc-bot_secret".to_string(), auth_info.secret.clone()),
      ]),
    }
  }

  /// set villa id
  pub fn with_villa_id(mut self, villa_id: u64) -> Self {
    self
      .headers
      .insert("x-rpc-bot_villa_id".to_string(), villa_id.to_string());
    self
  }

  /// create a iterator
  pub fn iter(&self) -> Iter<'_, String, String> {
    self.headers.iter()
  }
}

impl Debug for HeaderBuilder {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("HeaderBuilder")
      .field("x-rpc-bot_id", &self.headers.get("x-rpc-bot_id"))
      .field("x-rpc-bot_secret", &"***")
      .field(
        "x-rpc-bot_villa_id",
        &self.headers.get("x-rpc-bot_villa_id"),
      )
      .finish()
  }
}

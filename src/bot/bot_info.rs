use std::env::VarError;
use std::fmt::{Debug, Formatter};

/// Authentication information
pub struct BotAuthInfo {
  /// bot_id
  pub id: String,
  /// bot_secret
  pub secret: String,
}

impl BotAuthInfo {
  /// create a instance
  pub fn new(id: impl Into<String>, secret: impl Into<String>) -> Self {
    Self {
      id: id.into(),
      secret: secret.into(),
    }
  }

  /// create a instance from environment variable
  /// VILLA_BOT_ID, VILLA_BOT_SECRET
  pub fn from_env() -> Result<Self, VarError> {
    let id = std::env::var("VILLA_BOT_ID")?;
    let secret = std::env::var("VILLA_BOT_SECRET")?;
    Ok(Self { id, secret })
  }
}

impl Debug for BotAuthInfo {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("BotAuthInfo")
      .field("id", &self.id)
      .field("secret", &"***")
      .finish()
  }
}

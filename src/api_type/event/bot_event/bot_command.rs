use serde::Deserialize;

/// bot command
#[derive(Debug, Clone, Deserialize)]
pub struct BotCommand {
  /// name
  pub name: String,
  /// description
  #[serde(rename = "desc")]
  pub description: String,
}

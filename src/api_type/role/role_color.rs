use serde::{Deserialize, Serialize};

/// role color
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(from = "String", into = "String")]
pub enum RoleColor {
  /// #EB7F89
  Owner,
  /// #8F9BBF
  AllMember,
  /// #6173AB
  DarkBlue,
  /// #F485D8
  LightPink,
  /// #F47884
  Red,
  /// #FFA54B
  Orange,
  /// #7BC26F
  LightGreen,
  /// #59A1EA
  LightBlue,
  /// #977EE1
  Purple,
  /// unknown color
  Unknown(String),
}

impl From<String> for RoleColor {
  fn from(value: String) -> Self {
    let value = value.to_ascii_uppercase();

    match value.as_str() {
      "#EB7F89" => Self::Owner,
      "#8F9BBF" => Self::AllMember,
      "#6173AB" => Self::DarkBlue,
      "#F485D8" => Self::LightPink,
      "#F47884" => Self::Red,
      "#FFA54B" => Self::Orange,
      "#7BC26F" => Self::LightGreen,
      "#59A1EA" => Self::LightBlue,
      "#977EE1" => Self::Purple,
      _ => Self::Unknown(value),
    }
  }
}

impl From<RoleColor> for String {
  fn from(value: RoleColor) -> Self {
    match value {
      RoleColor::Owner => "#EB7F89".to_string(),
      RoleColor::AllMember => "#8F9BBF".to_string(),
      RoleColor::DarkBlue => "#6173AB".to_string(),
      RoleColor::LightPink => "#F485D8".to_string(),
      RoleColor::Red => "#F47884".to_string(),
      RoleColor::Orange => "#FFA54B".to_string(),
      RoleColor::LightGreen => "#7BC26F".to_string(),
      RoleColor::LightBlue => "#59A1EA".to_string(),
      RoleColor::Purple => "#977EE1".to_string(),
      RoleColor::Unknown(color) => color,
    }
  }
}

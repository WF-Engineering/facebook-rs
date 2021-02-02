use serde::{Deserialize, Serialize};
use std::string::ToString;

/// {
///   "data": {
///     "app_id": "745754449490053",
///     "type": "USER",
///     "application": "測試 WoWFood 登入",
///     "data_access_expires_at": 1608633182,
///     "expires_at": 1600862400,
///     "is_valid": true,
///     "scopes": [
///       "email",
///       "public_profile"
///     ],
///     "user_id": "10221538057473907"
///   }
/// }
#[derive(Debug, Deserialize, Serialize)]
pub struct AppTokenResp {
  pub access_token: String,
  token_type: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AccessTokenResp {
  pub access_token: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CheckTokenResp {
  data: TokenData,
}

impl CheckTokenResp {
  pub fn is_valid(&self, app_id: String) -> bool {
    let data = self.data.clone();
    data.is_valid
      && (Mode::from(data.mode) == Mode::User)
      && (data.app_id == app_id)
      && (data.scopes.contains(&ScopeItem::Email.to_string()))
      && (data.scopes.contains(&ScopeItem::PublicProfile.to_string()))
  }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TokenData {
  app_id: String,
  #[serde(rename = "type")]
  mode: String,
  application: String,
  data_access_expires_at: i64,
  expires_at: i64,
  is_valid: bool,
  scopes: Vec<String>,
  user_id: String,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
enum Mode {
  User,
  Others,
}

impl From<String> for Mode {
  fn from(mode: String) -> Self {
    match mode.as_str() {
      "USER" => Mode::User,
      _ => Mode::Others,
    }
  }
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Display)]
#[strum(serialize_all = "snake_case")]
enum ScopeItem {
  Email,
  PublicProfile,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UserFacebookData {
  pub id: String,
  pub name: String,
  pub email: String,
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn scope_item_from_str() {
    assert_eq!(Mode::from("USER".to_string()), Mode::User);
    assert_eq!(Mode::from("RANDOM".to_string()), Mode::Others);
  }

  #[test]
  fn mode_from_str() {
    assert_eq!(ScopeItem::Email.to_string(), "email".to_string());
    assert_eq!(
      ScopeItem::PublicProfile.to_string(),
      "public_profile".to_string()
    );
  }
}

extern crate strum;

#[macro_use]
extern crate strum_macros;

mod client;
mod models;

use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};
use reqwest::Result;

use crate::{client::*, models::*};

pub type FacebookApi = client::Client;

const URI_ENCODING: &AsciiSet = &CONTROLS
  .add(b' ')
  .add(b'!')
  .add(b'#')
  .add(b'$')
  .add(b'&')
  .add(b'\'')
  .add(b'(')
  .add(b')')
  .add(b'*')
  .add(b'+')
  .add(b',')
  .add(b'/')
  .add(b':')
  .add(b';')
  .add(b'=')
  .add(b'?')
  .add(b'@')
  .add(b'[')
  .add(b']')
  .add(b'.');

impl Client {
  pub async fn get_app_access_token(
    &self,
    app_id: &str,
    client_secret: &str,
  ) -> Result<AppTokenResp> {
    let path = format!(
      "/oauth/access_token?client_id={}&client_secret={}&grant_type=client_credentials",
      app_id, client_secret
    );
    let url = format!("{}{}", self.server_url, path);

    reqwest::Client::new().get(&url).send().await?.json().await
  }

  pub async fn get_access_token(
    &self,
    client_id: &str,
    client_secret: &str,
    redirect_uri: &str,
    code: &str,
  ) -> Result<AccessTokenResp> {
    let code = utf8_percent_encode(code, URI_ENCODING).to_string();
    let redirect_uri =
      utf8_percent_encode(redirect_uri, URI_ENCODING).to_string();

    let path = format!(
      "/oauth/access_token?client_id={client_id}&redirect_uri={redirect_uri}&client_secret={client_secret}&code={code}",
      client_id = client_id,
      redirect_uri = redirect_uri,
      client_secret = client_secret,
      code = code,
    );
    let url = format!("{}{}", self.server_url, path);

    reqwest::Client::new().get(&url).send().await?.json().await
  }
}

impl Client {
  pub async fn check_access_token(
    &self,
    user_access_token: &str,
    app_access_token: &str,
  ) -> Result<CheckTokenResp> {
    let path = format!(
      "/debug_token?input_token={}&access_token={}",
      user_access_token, app_access_token
    );
    let url = format!("{}{}", self.server_url, path);

    reqwest::Client::new().get(&url).send().await?.json().await
  }
}

impl Client {
  pub async fn get_user_data(
    &self,
    access_token: &str,
  ) -> Result<UserFacebookData> {
    let path = format!(
      "/v8.0/me?fields=id%2Cname%2Cemail&access_token={}",
      access_token
    );
    let url = format!("{}{}", self.server_url, path);

    reqwest::Client::new().get(&url).send().await?.json().await
  }
}

#[cfg(test)]
mod tests {
  use crate::FacebookApi;
  use log::debug;

  const ACCESSTOKEN: &str = "temp";
  const APPACCESSTOKEN: &str = "temp";

  #[tokio::test]
  async fn test_check_access_token_happy_path() {
    std::env::set_var("RUST_LOG", "debug");
    let _ = env_logger::builder().is_test(true).try_init();

    let facebook_client = FacebookApi::default();

    let result = facebook_client
      .check_access_token(ACCESSTOKEN, APPACCESSTOKEN)
      .await;
    debug!("{:?}", result);
    assert!(result.is_ok());
  }

  #[tokio::test]
  async fn test_get_user_data_happy_path() {
    std::env::set_var("RUST_LOG", "debug");
    let _ = env_logger::builder().is_test(true).try_init();

    let facebook_client = FacebookApi::default();

    let result = facebook_client.get_user_data(ACCESSTOKEN).await;
    debug!("{:?}", result);
    assert!(result.is_ok());
  }
}

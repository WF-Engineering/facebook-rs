#[derive(Debug, Clone)]
pub struct Client {
  pub server_url: String,
}

impl Default for Client {
  fn default() -> Self {
    let base_url = "https://graph.facebook.com".to_string();

    Self {
      server_url: base_url,
    }
  }
}

impl Client {
  pub fn new(server_url: String) -> Self {
    Self { server_url }
  }
}

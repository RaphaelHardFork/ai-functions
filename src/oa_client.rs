use async_openai::{config::OpenAIConfig, Client};
use std::sync::Arc;

use crate::Result;

pub type OaClient = Arc<Client<OpenAIConfig>>;

pub fn new_oa_client() -> Result<OaClient> {
    Ok(Client::new().into())
}

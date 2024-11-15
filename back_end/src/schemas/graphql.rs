use std::sync::Arc;
use awc::Client;

pub struct Context {
    pub client: Arc<Client>,
}

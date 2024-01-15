








use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Config {
    pub rpc_url: String,
    pub payer_secret: String,
    pub metadata_uri:String,
    pub merkle_tree: String,
    pub amount: u64
}
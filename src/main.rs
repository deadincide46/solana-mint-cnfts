
use log::{info, error};

use crate::{addons::config::read_config, transactions::{create_tree::create_tree_function, mint_nft::mint_cnft}};

pub mod logger;
pub mod addons;
pub mod structs;
pub mod transactions;

#[tokio::main]
async fn main() {
    logger::initialize_logger().expect("logger error");
    let config = read_config();
    info!("{:?}", config);
    let tree_result = create_tree_function(config.payer_secret.clone(), config.rpc_url.clone()).await;
    match tree_result {
        Ok(signature) => {
            info!("Create tree signature: {:?}", signature);
            mint_cnft(config.rpc_url, config.merkle_tree, config.payer_secret, config.amount, config.metadata_uri).await;
        }
        Err(error) => {
            error!("Create tree error: {:?}", error)
        }
    }

}

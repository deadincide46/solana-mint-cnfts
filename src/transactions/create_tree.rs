use log::info;
use mpl_bubblegum::{accounts::TreeConfig, instructions::CreateTreeConfigBuilder};
use solana_client::{rpc_client::RpcClient, client_error::ClientError};
use solana_sdk::{signature::{Keypair, Signature}, signer::Signer, system_instruction, transaction::{Transaction, VersionedTransaction}};
use spl_account_compression::{state::CONCURRENT_MERKLE_TREE_HEADER_SIZE_V1, ConcurrentMerkleTree};

use crate::addons::config::{read_config, write_config};






pub async fn create_tree_function(payer_secret:String, rpc_url:String) ->Result<Signature, ClientError> {
    let mut config = read_config();
    let rpc_client = RpcClient::new(rpc_url);
    let payer = Keypair::from_base58_string(&payer_secret);

    let merkle_tree = Keypair::new();
    info!("Merkle tree keypair: {:?}", merkle_tree.to_base58_string());
    config.merkle_tree = merkle_tree.to_base58_string();
    write_config(&config);
    let (tree_config, _) = TreeConfig::find_pda(&merkle_tree.pubkey());


    const MAX_DEPTH: usize = 14;
    const MAX_BUFFER_SIZE: usize = 64;

    let space = CONCURRENT_MERKLE_TREE_HEADER_SIZE_V1 + std::mem::size_of::<ConcurrentMerkleTree<MAX_DEPTH ,MAX_BUFFER_SIZE>>();
    loop {
        match rpc_client.get_minimum_balance_for_rent_exemption(space) {
            Ok(lamports) => {
                let create_account_ix = system_instruction::create_account(&payer.pubkey(), &merkle_tree.pubkey(), lamports, space as u64, &spl_account_compression::ID);
                let create_tree_config_ix = CreateTreeConfigBuilder::new().tree_config(tree_config).merkle_tree(merkle_tree.pubkey()).payer(payer.pubkey()).tree_creator(payer.pubkey()).max_depth(MAX_DEPTH as u32).max_buffer_size(MAX_BUFFER_SIZE as u32).instruction();
                loop {
                    let last_blockhash = rpc_client.get_latest_blockhash();
                    match last_blockhash {
                        Ok(hash) => {
                            let non_versioned_tx = Transaction::new_signed_with_payer(&[create_account_ix, create_tree_config_ix], Some(&payer.pubkey()), &[&merkle_tree, &payer], hash);
                            let versioned_tx = VersionedTransaction::from(non_versioned_tx);
                            let send_transaction = rpc_client.send_transaction(&versioned_tx);
                            return send_transaction;
                        }
                        Err(_error) => {
                            continue;
                        }
                    }
                }
            }
            Err(_error) => {
                //error getting lamports
            }
        }
    }
}





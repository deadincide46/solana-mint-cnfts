use std::{time::Duration, thread};

use log::{info, error};
use mpl_bubblegum::{accounts::TreeConfig, types::{MetadataArgs, Creator, TokenProgramVersion, TokenStandard}, instructions::MintV1Builder};
use solana_client::rpc_client::RpcClient;
use solana_sdk::{signature::Keypair, signer::Signer, transaction::{Transaction, VersionedTransaction}};










pub async fn mint_cnft(rpc_url:String, merkle_tree:String, payer_secret:String, amount:u64, metadata_uri:String) {
    let rpc_client = RpcClient::new(rpc_url);
    let merkle_tree = Keypair::from_base58_string(&merkle_tree);
    let (tree_config, _) = TreeConfig::find_pda(&merkle_tree.pubkey());

    for _ in 0..amount {
        let payer = Keypair::from_base58_string(&payer_secret); //wallet to mint nft
        let metadata = MetadataArgs {
            name: String::from("Test cNFT"),
            uri: String::from(&metadata_uri),
            symbol: String::from("Test cNFT"),
            creators: vec![Creator {
                address: payer.pubkey(),
                share: 100,
                verified: false,
            }],
            edition_nonce: None,
            is_mutable: true,
            primary_sale_happened: true,
            seller_fee_basis_points: 500,
            token_program_version: TokenProgramVersion::Original,
            token_standard: Some(TokenStandard::NonFungible),
            collection: None,
            uses: None,
        };
        loop {
            let last_blockhash = rpc_client.get_latest_blockhash();
            match last_blockhash {
                Ok(hash) => {
                    let mint_ix = MintV1Builder::new().leaf_delegate(payer.pubkey()).leaf_owner(payer.pubkey()).merkle_tree(merkle_tree.pubkey()).payer(payer.pubkey()).tree_config(tree_config).tree_creator_or_delegate(payer.pubkey()).metadata(metadata).instruction();
                    let mint_non_versioned_tx = Transaction::new_signed_with_payer(&[mint_ix], Some(&payer.pubkey()), &[&payer], hash);
                    let mint_versioned_tx =  VersionedTransaction::from(mint_non_versioned_tx);
                
                    let mint_send_transaction = rpc_client.send_transaction(&mint_versioned_tx);
                    match mint_send_transaction {
                        Ok(signature) => {
                            info!("Success mint to: {:?}. Signature: {:?}", payer.pubkey(), signature);
                            let duration = Duration::from_secs(1);
                            thread::sleep(duration);
                        }
                        Err(error) => {
                            error!("Error while trying mint: {:?}", error);
                            let duration = Duration::from_secs(1);
                            thread::sleep(duration);
                        }
                    }
                    break;
                }
                Err(_error) => {
                    //will try again getting blockhash for trans
                    continue;
                }
            }
        }
    }
}
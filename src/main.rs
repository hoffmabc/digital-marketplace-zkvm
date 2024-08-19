pub mod constants;
pub mod helpers;

use crate::constants::*;
use bitcoin::key::rand::rngs::OsRng;
use bitcoin::XOnlyPublicKey;
use bitcoin::{
    key::{Parity, Secp256k1, UntweakedKeypair},
    secp256k1::SecretKey,
    Address,
};
use digital_marketplace::state::MarketPlaceState;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::{fs, str::FromStr};

fn main() {
    match deploy_program() {
        Ok(program_id) => match hex::decode(&program_id) {
            Ok(_) => println!("Program deployed successfully. Program ID: {}", program_id),
            Err(e) => println!("Program deployed, but failed to decode program ID: {}", e),
        },
        Err(e) => println!("Failed to deploy program: {}", e),
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DeployProgramParams {
    elf: Vec<u8>,
}

fn check_program_exists(program_id: &str) -> Result<bool, anyhow::Error> {
    let arch_node_url = "http://127.0.0.1:9001/";
    let client = reqwest::blocking::Client::new();
    let res = client
        .post(arch_node_url)
        .header("content-type", "application/json")
        .json(&json!({
            "jsonrpc": "2.0",
            "id": "curlycurl",
            "method": "get_program",
            "params": [program_id]
        }))
        .send()?;

    let result: serde_json::Value = serde_json::from_str(&res.text()?)?;
    Ok(result["result"].is_object())
}

fn deploy_program() -> Result<String, anyhow::Error> {
    let arch_node_url = "http://127.0.0.1:9001/";
    let elf = fs::read("target/digital-marketplace.elf")?;
    let params = DeployProgramParams { elf };

    // First, attempt to deploy the program
    let client = reqwest::blocking::Client::new();
    let res = client
        .post(arch_node_url)
        .header("content-type", "application/json")
        .json(&json!({
            "jsonrpc": "2.0",
            "id": "curlycurl",
            "method": "deploy_program",
            "params": params
        }))
        .send()?;

    let result: serde_json::Value = serde_json::from_str(&res.text()?)?;
    let program_id = result["result"]
        .as_str()
        .ok_or_else(|| anyhow::anyhow!("No result found"))?
        .to_string();

    // Check if the program already existed
    if check_program_exists(&program_id)? {
        println!("Program already exists with ID: {}", program_id);
    } else {
        println!("Program deployed successfully with ID: {}", program_id);
    }

    Ok(program_id)
}

/// Represents a party or node secret and address information
pub struct CallerInfo {
    pub key_pair: UntweakedKeypair,
    pub public_key: XOnlyPublicKey,
    pub parity: Parity,
    pub address: Address,
}

impl CallerInfo {
    /// Create a [CallerInfo] from the specified file path
    /// If the file does not exist, generate a random secret key
    /// and use that instead.
    pub fn with_secret_key_file(file_path: &str) -> Result<CallerInfo, anyhow::Error> {
        let secp = Secp256k1::new();
        let secret_key = match fs::read_to_string(file_path) {
            Ok(key) => SecretKey::from_str(&key).unwrap(),
            Err(_) => {
                let (key, _) = secp.generate_keypair(&mut OsRng);
                fs::write(file_path, &key.display_secret().to_string())
                    .map_err(|_| anyhow::anyhow!("Unable to write file"))?;
                key
            }
        };
        let key_pair = UntweakedKeypair::from_secret_key(&secp, &secret_key);
        let (public_key, parity) = key_pair.x_only_public_key();
        let address = Address::p2tr(&secp, public_key, None, bitcoin::Network::Regtest);
        Ok(CallerInfo {
            key_pair,
            public_key,
            parity,
            address,
        })
    }
}

#[cfg(test)]
mod marketplace_tests {
    use std::str::FromStr;

    use borsh::{BorshDeserialize, BorshSerialize};
    use helpers::{get_processed_transaction, read_utxo, send_utxo, sign_and_send_instruction};
    use sdk::{Pubkey, UtxoMeta};

    use super::*;

    #[derive(Clone, BorshSerialize, BorshDeserialize)]
    pub struct MarketplaceParams {
        instruction: MarketplaceInstruction,
        tx_hex: Vec<u8>,
    }

    #[derive(Clone, BorshSerialize, BorshDeserialize)]
    pub enum MarketplaceInstruction {
        CreateUser { username: String },
        ListItem { name: String, price: u64 },
        PurchaseItem { item_id: u64 },
        UpdateItem { item_id: u64, new_price: u64 },
    }

    fn setup() -> (Pubkey, String) {
        println!("Starting setup...");

        println!("Deploying program...");
        let deploy_result = deploy_program();
        println!("Deploy program result: {:?}", deploy_result);

        let deployed_program_id = match deploy_result {
            Ok(id_str) => match Pubkey::from_str(&id_str) {
                Ok(pubkey) => {
                    println!("Successfully created Pubkey from deploy result");
                    pubkey
                }
                Err(e) => {
                    println!("Error creating Pubkey from deploy result: {:?}", e);
                    panic!("Failed to create Pubkey from deploy result: {:?}", e);
                }
            },
            Err(e) => {
                println!("Error deploying program: {:?}", e);
                panic!("Failed to deploy program: {:?}", e);
            }
        };

        println!("Deployed program ID: {:?}", deployed_program_id);

        println!("Sending UTXO...");
        let state_txid = send_utxo();
        println!("State TXID: {}", state_txid);

        println!("Setup completed successfully");
        (deployed_program_id, state_txid)
    }

    #[test]
    fn test_create_user() {
        let (deployed_program_id, state_txid) = setup();

        let instruction_data = MarketplaceParams {
            instruction: MarketplaceInstruction::CreateUser {
                username: "Alice".to_string(),
            },
            tx_hex: match hex::decode(prepare_fees()) {
                Ok(hex) => hex,
                Err(e) => panic!("Failed to decode hex string: {}", e),
            },
        };
        let instruction_data = borsh::to_vec(&instruction_data).unwrap();

        println!("Instruction data: {:?}", instruction_data);

        let (txid, instruction_hash) = sign_and_send_instruction(
            deployed_program_id,
            vec![UtxoMeta {
                txid: state_txid.clone(),
                vout: 1,
            }],
            instruction_data,
        )
        .unwrap();

        println!(
            "Signed and sent instruction. TXID: {}, Instruction Hash: {}",
            txid, instruction_hash
        );

        // Add a delay or retry mechanism here if necessary
        std::thread::sleep(std::time::Duration::from_secs(10));

        let processed_tx = get_processed_transaction(NODE1_ADDRESS, txid).unwrap();
        println!("Processed transaction: {:?}", processed_tx);

        let new_state_txid = processed_tx.bitcoin_txids[&instruction_hash].clone();
        println!("New state TXID: {}", new_state_txid);

        let utxo = read_utxo(NODE1_ADDRESS, format!("{}:0", new_state_txid)).unwrap();
        println!("UTXO data: {:?}", utxo);

        // Deserialize the UTXO data using your MarketPlaceState structure
        let state = MarketPlaceState::deserialize(&utxo.data).unwrap();

        // Assert that the user was created
        assert_eq!(state.users.len(), 1, "Expected one user to be created");
        assert_eq!(
            state.users[0].username, "Alice",
            "Expected user with username 'Alice'"
        );

        // Add more assertions as necessary
        println!(
            "Test completed successfully. State after user creation: {:?}",
            state
        );
    }

    #[test]
    fn test_list_item() {
        let (_deployed_program_id, _state_txid) = setup();

        let instruction_data = MarketplaceParams {
            instruction: MarketplaceInstruction::ListItem {
                name: "Vintage Watch".to_string(),
                price: 1000,
            },
            tx_hex: hex::decode(prepare_fees()).unwrap(),
        };
        let _instruction_data = borsh::to_vec(&instruction_data).unwrap();

        // Similar process to test_create_user
        // ...

        // Assert the item has been listed correctly
    }

    #[test]
    fn test_purchase_item() {
        // First, list an item
        // Then, purchase the item
        // Assert the purchase was successful and the state updated correctly
    }

    #[test]
    fn test_update_item() {
        // First, list an item
        // Then, update the item's price
        // Assert the item was updated correctly
    }

    #[test]
    fn test_multiple_operations() {
        // Perform a series of operations (create user, list item, purchase item)
        // Assert the final state is correct after all operations
    }

    // Helper function to prepare fees (assuming this is needed)
    fn prepare_fees() -> String {
        // Return a valid hex string representing some dummy fees
        "0000000000000000".to_string()
    }
}

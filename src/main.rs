use borsh::{BorshDeserialize, BorshSerialize};
use digital_marketplace_program::{models::*, processor::*};
use sdk::{Pubkey, UtxoInfo};
use std::cell::RefCell;

fn main() {
    println!("Digital Marketplace Program");

    // Create a dummy Pubkey and UTXOs for testing
    let dummy_pubkey = Pubkey::new_unique();
    let mut dummy_utxos = vec![UtxoInfo {
        data: RefCell::new(Vec::new()),
        authority: RefCell::new(dummy_pubkey),
        txid: [0; 32], // Add a dummy txid
        vout: 0,       // Add a dummy vout
    }];

    // Create a user
    let create_user_result = create_user(
        CreateUserParams {
            name: "Alice".to_string(),
            initial_balance: 1000,
        },
        &dummy_pubkey,
        &dummy_utxos,
    )
    .expect("Failed to create user");

    println!("Created user: {:?}", create_user_result);

    // Update dummy_utxos with the new state
    let mut serialized_state = Vec::new();
    create_user_result
        .serialize(&mut serialized_state)
        .expect("Failed to serialize state");
    *dummy_utxos[0].data.borrow_mut() = serialized_state;

    // List an item
    let list_item_result = list_item(
        ListItemParams {
            name: "Book".to_string(),
            description: "A great book".to_string(),
            price: 50,
        },
        &dummy_pubkey,
        &dummy_utxos,
    )
    .expect("Failed to list item");

    println!("Listed item: {:?}", list_item_result);

    // Update dummy_utxos with the new state
    let mut serialized_state = Vec::new();
    list_item_result
        .serialize(&mut serialized_state)
        .expect("Failed to serialize state");
    *dummy_utxos[0].data.borrow_mut() = serialized_state;

    // Simulate a purchase
    let purchase_result = purchase_item(
        PurchaseItemParams {
            item_id: "item1".to_string(), // Assuming this is the ID generated for the book
        },
        &dummy_pubkey,
        &dummy_utxos,
    )
    .expect("Failed to purchase item");

    println!("Purchase result: {:?}", purchase_result);

    // Print final state
    println!("Final marketplace state: {:?}", purchase_result);
}

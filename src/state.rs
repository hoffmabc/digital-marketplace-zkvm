use borsh::{BorshDeserialize, BorshSerialize};
use sdk::Pubkey;
use serde::{Deserialize, Serialize};

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Debug, Clone)]
pub struct MarketplaceState {
    pub users: Vec<User>,
    pub items: Vec<Item>,
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Debug, Clone)]
pub struct User {
    pub id: String,
    pub name: String,
    pub balance: u64,
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Debug, Clone)]
pub struct Item {
    pub id: String,
    pub name: String,
    pub description: String,
    pub price: u64,
    pub seller: Pubkey,
    pub available: bool,
}

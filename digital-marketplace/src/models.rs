use borsh::{BorshDeserialize, BorshSerialize};
use sdk::Pubkey;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[derive(Clone, BorshSerialize, BorshDeserialize, Serialize, Deserialize, Debug)]
pub struct User {
    pub id: String,
    pub username: String,
    pub balance: u64,
}

#[derive(Clone, BorshSerialize, BorshDeserialize, Serialize, Deserialize, Debug)]
pub struct Item {
    pub id: String,
    pub name: String,
    pub description: String,
    pub price: u64,
    pub seller: Pubkey,
    pub available: bool,
}

#[derive(Clone, BorshSerialize, BorshDeserialize)]
pub enum MarketplaceInstruction {
    CreateUser(CreateUserParams),
    ListItem(ListItemParams),
    PurchaseItem(PurchaseItemParams),
    UpdateItem(UpdateItemParams),
}

#[derive(Clone, BorshSerialize, BorshDeserialize)]
pub struct CreateUserParams {
    pub username: String,
    pub initial_balance: u64,
    pub tx_hex: Vec<u8>,
}

#[derive(Clone, BorshSerialize, BorshDeserialize)]
pub struct ListItemParams {
    pub name: String,
    pub description: String,
    pub price: u64,
    pub tx_hex: Vec<u8>,
}

#[derive(Clone, BorshSerialize, BorshDeserialize)]
pub struct PurchaseItemParams {
    pub item_id: String,
    pub tx_hex: Vec<u8>,
}

#[derive(Clone, BorshSerialize, BorshDeserialize)]
pub struct UpdateItemParams {
    pub item: Item,
    pub tx_hex: Vec<u8>,
}

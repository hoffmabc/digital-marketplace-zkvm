use borsh::{BorshDeserialize, BorshSerialize};

#[derive(Clone, BorshSerialize, BorshDeserialize, Debug)]
pub struct Item {
    pub id: String,
    pub name: String,
    pub description: String,
    pub price: u32,
    pub seller: String,
    pub available: bool,
}

#[derive(Clone, BorshSerialize, BorshDeserialize, Debug)]
pub struct User {
    pub id: String,
    pub name: String,
    pub balance: u32,
}

#[derive(Clone, BorshSerialize, BorshDeserialize, Debug)]
pub struct CreateUserParams {
    pub id: String,
    pub name: String,
    pub initial_balance: u32,
    pub tx_hex: Vec<u8>,
}

#[derive(Clone, BorshSerialize, BorshDeserialize, Debug)]
pub struct ListItemParams {
    pub item: Item,
    pub tx_hex: Vec<u8>,
}

#[derive(Clone, BorshSerialize, BorshDeserialize, Debug)]
pub struct PurchaseItemParams {
    pub item_id: String,
    pub buyer_id: String,
    pub tx_hex: Vec<u8>,
}

#[derive(Clone, BorshSerialize, BorshDeserialize, Debug)]
pub struct UpdateItemParams {
    pub item: Item,
    pub tx_hex: Vec<u8>,
}

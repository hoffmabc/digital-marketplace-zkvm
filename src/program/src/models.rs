use borsh::{BorshDeserialize, BorshSerialize};
use sdk::Pubkey;

#[derive(Clone, BorshSerialize, BorshDeserialize, Debug)]
pub struct Item {
    pub id: String,
    pub name: String,
    pub description: String,
    pub price: u64,
    pub seller: Pubkey,
    pub available: bool,
}

#[derive(Clone, BorshSerialize, BorshDeserialize, Debug)]
pub struct User {
    pub id: Pubkey,
    pub name: String,
    pub balance: u64,
}

#[derive(Clone, BorshSerialize, BorshDeserialize, Debug)]
pub enum MarketplaceInstruction {
    CreateUser(CreateUserParams),
    ListItem(ListItemParams),
    PurchaseItem(PurchaseItemParams),
    UpdateItem(UpdateItemParams),
}

#[derive(Clone, BorshSerialize, BorshDeserialize, Debug)]
pub struct CreateUserParams {
    pub name: String,
    pub initial_balance: u64,
}

#[derive(Clone, BorshSerialize, BorshDeserialize, Debug)]
pub struct ListItemParams {
    pub name: String,
    pub description: String,
    pub price: u64,
}

#[derive(Clone, BorshSerialize, BorshDeserialize, Debug)]
pub struct PurchaseItemParams {
    pub item_id: String,
}

#[derive(Clone, BorshSerialize, BorshDeserialize, Debug)]
pub struct UpdateItemParams {
    pub item_id: String,
    pub new_price: Option<u64>,
    pub new_description: Option<String>,
    pub new_availability: Option<bool>,
}

#[derive(Clone, BorshSerialize, BorshDeserialize, Debug)]
pub struct MarketplaceState {
    pub users: Vec<User>,
    pub items: Vec<Item>,
}

impl Default for MarketplaceState {
    fn default() -> Self {
        MarketplaceState {
            users: Vec::new(),
            items: Vec::new(),
        }
    }
}

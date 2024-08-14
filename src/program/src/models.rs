use borsh::{BorshDeserialize, BorshSerialize};

#[derive(Clone, BorshSerialize, BorshDeserialize)]
pub struct Item {
    pub id: String,
    pub name: String,
    pub description: String,
    pub price: u32,
    pub seller: String,
    pub available: bool,
}

#[derive(Clone, BorshSerialize, BorshDeserialize)]
pub struct User {
    pub id: String,
    pub name: String,
    pub balance: u32,
}

#[derive(Clone, BorshSerialize, BorshDeserialize)]
pub enum MarketplaceInstruction {
    CreateUser(CreateUserParams),
    ListItem(ListItemParams),
    PurchaseItem(PurchaseItemParams),
    UpdateItem(UpdateItemParams),
}

impl MarketplaceInstruction {
    pub fn tx_hex(&self) -> Vec<u8> {
        match self {
            MarketplaceInstruction::CreateUser(inner) => inner.tx_hex.clone(),
            MarketplaceInstruction::ListItem(inner) => inner.tx_hex.clone(),
            MarketplaceInstruction::PurchaseItem(inner) => inner.tx_hex.clone(),
            MarketplaceInstruction::UpdateItem(inner) => inner.tx_hex.clone(),
        }
    }
}

#[derive(Clone, BorshSerialize, BorshDeserialize)]
pub struct CreateUserParams {
    pub id: String,
    pub name: String,
    pub initial_balance: u32,
    pub tx_hex: Vec<u8>,
}

#[derive(Clone, BorshSerialize, BorshDeserialize)]
pub struct ListItemParams {
    pub item: Item,
    pub tx_hex: Vec<u8>,
}

#[derive(Clone, BorshSerialize, BorshDeserialize)]
pub struct PurchaseItemParams {
    pub item_id: String,
    pub buyer_id: String,
    pub tx_hex: Vec<u8>,
}

#[derive(Clone, BorshSerialize, BorshDeserialize)]
pub struct UpdateItemParams {
    pub item: Item,
    pub tx_hex: Vec<u8>,
}

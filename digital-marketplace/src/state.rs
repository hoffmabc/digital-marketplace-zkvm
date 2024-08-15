use crate::models::{Item, User};
use borsh::{BorshDeserialize, BorshSerialize};
use serde::{Deserialize, Serialize};

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Debug, Clone)]
pub struct MarketPlaceState {
    pub users: Vec<User>,
    pub items: Vec<Item>,
}

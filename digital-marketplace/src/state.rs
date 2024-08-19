use crate::models::{Item, User};
use borsh::{BorshDeserialize, BorshSerialize};
use sdk::UtxoInfo;
use serde::{Deserialize, Serialize};

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Debug, Clone, Default)]
pub struct MarketPlaceState {
    pub users: Vec<User>,
    pub items: Vec<Item>,
}

impl MarketPlaceState {
    pub fn serialize(&self) -> Result<Vec<u8>, anyhow::Error> {
        borsh::to_vec(self).map_err(|e| anyhow::anyhow!("Serialization error: {}", e))
    }

    pub fn deserialize(data: &[u8]) -> Result<Self, anyhow::Error> {
        borsh::from_slice(data).map_err(|e| anyhow::anyhow!("Deserialization error: {}", e))
    }

    pub fn from_utxos(utxos: &[UtxoInfo]) -> Result<Self, anyhow::Error> {
        let mut state = Self::default();

        for utxo in utxos {
            let data = utxo.data.borrow();
            if !data.is_empty() {
                let utxo_state = Self::deserialize(&data)?;
                state.users.extend(utxo_state.users);
                state.items.extend(utxo_state.items);
            }
        }

        Ok(state)
    }
}

#![no_main]
use anyhow::Result;
use borsh::{BorshDeserialize, BorshSerialize};
use sdk::{entrypoint, Pubkey, UtxoInfo};
mod models;
mod processor;
use models::{MarketplaceInstruction, MarketplaceState};

entrypoint!(handler);

pub fn handler(
    program_id: &Pubkey,
    utxos: &[UtxoInfo],
    instruction_data: &[u8],
) -> Result<Vec<u8>> {
    let instruction: MarketplaceInstruction =
        BorshDeserialize::deserialize(&mut &instruction_data[..])?;

    let result: Result<MarketplaceState> = match instruction {
        MarketplaceInstruction::CreateUser(params) => {
            processor::create_user(params, program_id, utxos)
        }
        MarketplaceInstruction::ListItem(params) => processor::list_item(params, program_id, utxos),
        MarketplaceInstruction::PurchaseItem(params) => {
            processor::purchase_item(params, program_id, utxos)
        }
        MarketplaceInstruction::UpdateItem(params) => {
            processor::update_item(params, program_id, utxos)
        }
    };

    let updated_state = result?;
    let mut result_data = Vec::new();
    updated_state.serialize(&mut result_data)?;

    for utxo in utxos {
        *utxo.data.borrow_mut() = result_data.clone();
    }

    Ok(result_data)
}

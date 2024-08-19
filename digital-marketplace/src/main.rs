#![no_main]
use anyhow::Result;
use borsh::BorshDeserialize;
use digital_marketplace::{models::MarketplaceInstruction, processor, state::MarketPlaceState};
use sdk::{entrypoint, Pubkey, UtxoInfo};

#[cfg(target_os = "zkvm")]
entrypoint!(handler);

#[cfg(target_os = "zkvm")]
pub fn handler(
    program_id: &Pubkey,
    utxos: &[UtxoInfo],
    instruction_data: &[u8],
) -> Result<Vec<u8>> {
    let instruction: MarketplaceInstruction =
        BorshDeserialize::deserialize(&mut &instruction_data[..])?;
    let result: Result<MarketPlaceState> = match instruction {
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
    let result_data = Vec::new();

    updated_state.serialize()?;

    for utxo in utxos {
        *utxo.data.borrow_mut() = result_data.clone();
    }
    Ok(result_data)
}

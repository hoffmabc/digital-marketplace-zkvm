use crate::models::Item;
use crate::models::{CreateUserParams, ListItemParams, PurchaseItemParams, UpdateItemParams, User};
use crate::state::MarketPlaceState;
use anyhow::Result;
use sdk::{Pubkey, UtxoInfo};

pub fn create_user(
    params: CreateUserParams,
    signer: &Pubkey,
    utxos: &[UtxoInfo],
) -> Result<MarketPlaceState> {
    let mut state = deserialize_state(utxos)?;
    let new_user = User {
        id: signer.to_string(),
        username: params.username,
        balance: params.initial_balance,
    };
    state.users.push(new_user);
    Ok(state)
}

pub fn list_item(
    params: ListItemParams,
    signer: &Pubkey,
    utxos: &[UtxoInfo],
) -> Result<MarketPlaceState> {
    let mut state = deserialize_state(utxos)?;
    let new_item = Item {
        id: generate_item_id(),
        name: params.name,
        description: params.description,
        price: params.price,
        seller: signer.clone(),
        available: true,
    };
    state.items.push(new_item);
    Ok(state)
}

pub fn purchase_item(
    params: PurchaseItemParams,
    signer: &Pubkey,
    utxos: &[UtxoInfo],
) -> Result<MarketPlaceState> {
    let mut state = deserialize_state(utxos)?;
    let (buyer_index, seller_index, item_index) = {
        let buyer_index = state
            .users
            .iter()
            .position(|u| u.id == signer.to_string())
            .ok_or_else(|| anyhow::anyhow!("Buyer not found"))?;
        let item_index = state
            .items
            .iter()
            .position(|i| i.id == params.item_id)
            .ok_or_else(|| anyhow::anyhow!("Item not found"))?;
        let seller_id = &state.items[item_index].seller;
        let seller_index = state
            .users
            .iter()
            .position(|u| u.id == seller_id.to_string())
            .ok_or_else(|| anyhow::anyhow!("Seller not found"))?;
        (buyer_index, seller_index, item_index)
    };

    let (buyer, item) = {
        let (buyers, _items) = state.users.split_at_mut(buyer_index + 1);
        (&mut buyers[buyer_index], &mut state.items[item_index])
    };

    if !item.available || buyer.balance < item.price {
        return Err(anyhow::anyhow!("Invalid purchase conditions"));
    }

    buyer.balance = buyer.balance.saturating_sub(item.price);
    state.users[seller_index].balance =
        state.users[seller_index].balance.saturating_add(item.price);
    item.available = false;

    Ok(state)
}

pub fn update_item(
    params: UpdateItemParams,
    signer: &Pubkey,
    utxos: &[UtxoInfo],
) -> Result<MarketPlaceState> {
    let mut state = deserialize_state(utxos)?;
    let item = state
        .items
        .iter_mut()
        .find(|i| i.id == params.item.id)
        .ok_or_else(|| anyhow::anyhow!("Item not found"))?;
    if item.seller != *signer {
        return Err(anyhow::anyhow!("Only the seller can update the item"));
    }
    item.name = params.item.name;
    item.description = params.item.description;
    item.price = params.item.price;
    item.available = params.item.available;
    Ok(state)
}

pub fn deserialize_state(utxos: &[UtxoInfo]) -> Result<MarketPlaceState> {
    MarketPlaceState::from_utxos(utxos)
}

fn generate_item_id() -> String {
    use rand::Rng;
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            abcdefghijklmnopqrstuvwxyz\
                            0123456789";
    const ID_LEN: usize = 16;
    let mut rng = rand::thread_rng();

    (0..ID_LEN)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect()
}

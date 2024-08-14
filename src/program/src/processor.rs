use crate::models::{
    CreateUserParams, Item, ListItemParams, PurchaseItemParams, UpdateItemParams, User,
};

pub fn create_user(params: CreateUserParams) -> User {
    User {
        id: params.id,
        name: params.name,
        balance: params.initial_balance,
    }
}

pub fn list_item(params: ListItemParams) -> Item {
    params.item
}

pub fn purchase_item(
    params: PurchaseItemParams,
    item: &mut Item,
    buyer: &mut User,
    seller: &mut User,
) -> bool {
    if !item.available
        || buyer.balance < item.price
        || buyer.id != params.buyer_id
        || item.id != params.item_id
    {
        return false;
    }

    buyer.balance = buyer.balance.saturating_sub(item.price);
    seller.balance = seller.balance.saturating_add(item.price);
    item.available = false;
    true
}

pub fn update_item(params: UpdateItemParams, existing_item: &mut Item) -> bool {
    if params.item.id != existing_item.id || params.item.seller != existing_item.seller {
        return false;
    }

    *existing_item = params.item;
    true
}

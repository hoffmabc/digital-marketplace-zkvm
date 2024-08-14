use crate::models::{
    CreateUserParams, Item, ListItemParams, PurchaseItemParams, UpdateItemParams, User,
};
use std::fmt;

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

// Debug wrappers for functions
pub struct CreateUserCall(pub CreateUserParams);
pub struct ListItemCall(pub ListItemParams);
pub struct PurchaseItemCall(pub PurchaseItemParams, pub Item, pub User, pub User);
pub struct UpdateItemCall(pub UpdateItemParams, pub Item);

impl fmt::Debug for CreateUserCall {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "CreateUserCall({:?})", self.0)
    }
}

impl fmt::Debug for ListItemCall {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ListItemCall({:?})", self.0)
    }
}

impl fmt::Debug for PurchaseItemCall {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "PurchaseItemCall({:?}, {:?}, {:?}, {:?})",
            self.0, self.1, self.2, self.3
        )
    }
}

impl fmt::Debug for UpdateItemCall {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "UpdateItemCall({:?}, {:?})", self.0, self.1)
    }
}

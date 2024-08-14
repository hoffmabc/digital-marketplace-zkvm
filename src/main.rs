use digital_marketplace_program::{models::*, processor::*};

fn main() {
    println!("Digital Marketplace Program");

    // Create a user
    let user = create_user(CreateUserParams {
        id: "user1".to_string(),
        name: "Alice".to_string(),
        initial_balance: 1000,
        tx_hex: vec![],
    });
    println!("Created user: {:?}", user);

    // List an item
    let item = list_item(ListItemParams {
        item: Item {
            id: "item1".to_string(),
            name: "Book".to_string(),
            description: "A great book".to_string(),
            price: 50,
            seller: "user1".to_string(),
            available: true,
        },
        tx_hex: vec![],
    });
    println!("Listed item: {:?}", item);

    // Simulate a purchase
    let mut buyer = User {
        id: "user2".to_string(),
        name: "Bob".to_string(),
        balance: 100,
    };
    let mut seller = user.clone();
    let mut item_to_purchase = item.clone();

    let purchase_successful = purchase_item(
        PurchaseItemParams {
            item_id: "item1".to_string(),
            buyer_id: "user2".to_string(),
            tx_hex: vec![],
        },
        &mut item_to_purchase,
        &mut buyer,
        &mut seller,
    );

    println!("Purchase successful: {}", purchase_successful);

    println!("Updated buyer: {:?}", buyer);
    println!("Updated seller: {:?}", seller);
    println!("Updated item: {:?}", item_to_purchase);
}

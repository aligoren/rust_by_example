#[derive(Debug)]
struct OrderItem {
    product_name: String,
    price: f64,
    quantity: f64,
    user_id: u32
}

// Representing RGB
#[derive(Debug)]
struct Color(i32, i32, i32);

fn build_order_item(product_name: String, price: f64, quantity: f64, user_id: u32) -> OrderItem {
    OrderItem {
        product_name,
        price,
        quantity,
        user_id
    }
}

fn main() {
    // https://doc.rust-lang.org/book/ch05-00-structs.html

    let mut basket : Vec<&OrderItem> = Vec::new();

    let product_name = String::from("Algorithms");
    
    let order = OrderItem {
        product_name,
        price: 60.0,
        quantity: 1.0,
        user_id: 12851
    };

    let order2 = OrderItem {
        product_name: String::from("Network Programming"),
        price: 120.0,
        quantity: 2.0,
        user_id: 9867423
    };

    basket.push(&order);
    basket.push(&order2);

    let product_name = String::from("Rust by Example");

    let order_item = build_order_item(product_name, 45.0, 2.0, 19241);

    basket.push(&order_item);

    println!("Basket values {:#?}", basket);

    // salmon
    let rgb_color = Color(250,128,114);

    println!("The color is {:?}", rgb_color);

}

// Rust functions should be snake case. You don't have to but, it is recommended by Rust compiler.

fn hello_world() {
    println!("Hello World!");
}

// function with parameters
fn user_info(name: String, age: u8, vip: bool) {
    println!("Hi. I'm {} and I'm {} years old. My VIP member status is: {}", name, age, vip);
}

// returning a result
fn verify_age(age: u8) -> bool {
    return age > 13;
}

// if you don't use return keyword, it will return the last statement unless you use semi-colon
fn sum_two_value(number_one: i32, number_two: i32) -> i32 {
    number_one + 1 + number_two //don't use semi-colon
}

fn main() {
    hello_world();
    user_info(String::from("Ali"), 27, true);
    println!("Is age verified: {}", verify_age(21));
    println!("Total: {}", sum_two_value(1, 2));
}

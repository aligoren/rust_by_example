fn main() {
    // Variable scope
    let mut s = "hello";

    println!("Hi {}", s);

    {
        let s = "hello world!";
    }

    println!("S variable again {}", s);

    // string type
    let mut name_surname = String::from("Ali");

    // ups I forget my surname. Let me add

    name_surname.push_str(" GÖREN");

    println!("Now my friends can identify me. I'm {}", name_surname);

    let message = String::from("hello");

    // these lines will throw an error
    // let message2 = message;
    // println!("Hello {}", message);

    let message2 = message.clone();
    println!("message = {} and message2 = {}" ,message, message2);

    let mut username = String::from("Ali");

    username = set_username(username);

    println!("You passed {}", username); // it won't show error. because we gave back ownership

}

// set your username
fn set_username(name: String) -> String {
    println!("Username: {}", name);
    name
}
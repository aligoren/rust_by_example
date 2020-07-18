fn main() {

    // if expressions
    let number: i8 = 5;

    if number > 0 {
        println!("{} greater than 0", number);
    } else {
        println!("{} less than 0", number);
    }

    // if - else if expressions
    let number: i8 = -100;

    if number > 0 {
        println!("{} greater than 0", number);
    } else if number == 0 {
        println!("{} is equals to 0", number);
    } else {
        println!("{} less than 0", number);
    }

    // ternary if
    let number: i8 = 5;

    let condition: bool = if number == 5 { true } else { false };

    println!("Condition is {}", condition);

    // Repetition with Loops
    // don't uncomment below lines
    // loop {
    //     println!("Stop me if you can!");
    // }

    let mut counter: u64 = 0;
    
    let result = loop {
        counter += 1;

        if counter % 10 == 2 {
            println!("Loop breaked");
            break counter;
        }
    };

    println!("Current counter value {}", result);

    let mut number = 10;

    while number != 0 {
        println!("Number {}", number);

        number -= 1;
    }

    let users = ["Ali", "Can", "Ben", "Burak"];

    // we use iter because of ownership-borrowing
    for user in users.iter() {
        println!("User: {}", user);
    }

    for number in (1..5).rev() {
        println!("Number is {}", number);
    }


}

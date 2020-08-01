fn main() {
    let my_name = String::from("Ali");

    //this ampersand is a reference
    let len_name = name_length(&my_name);
    //  -----------------------^

    println!("The length of {} is {}", my_name, len_name);


    // this won't work. because resources isn't immutable
    // the function expects immutable

    // let say_hey = String::from("Hey!");
    //borrowing_immutable_example(&say_hey);

    // this will work. because resource is a mutable variable
    // function expects mutable variable

    let mut say_hey = String::from("Hey!");
    let say_hey_cloned = say_hey.clone();
    let mutable_result = borrowing_mutable_example(&mut say_hey);

    println!("I passed {} and the function returned {}", say_hey_cloned, mutable_result);

    let mut a_number = 5;

    {
        // We're getting mutable reference
        let y = &mut a_number;
        // We're increasing the value of resource which point from reference.
        *y += 1;
    }

    // a_number is 6 now. Because we used reference in the scope we've created.
    println!("A number is {}", a_number);
}

// this ampersand is a reference
fn name_length(string: &String) -> usize {
    // ----------------^
    string.len()
}

// this won't work
// help: consider changing this to be a mutable reference: `&mut std::string::String`

/*fn borrowing_immutable_example(a_string: &String) {
    a_string.push_str(", what?");
}*/

// to fix immutable reference error, use mutable references
fn borrowing_mutable_example(a_string: &mut String) -> &String {
    a_string.push_str(", what?");

    a_string
}
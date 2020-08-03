fn first_word(a_str: &String) -> usize {

    let bytes = a_str.as_bytes();

    for (index, &byte) in bytes.iter().enumerate() {
        if byte == b' ' {
            println!("Loop breaked! {}", index);
            return index;
        }
    }

    

    a_str.len()
}

fn main() {
    let mut s = String::from("Hello world!");

    let _ = first_word(&s);

    println!("Result {:?}", &s[5..]);
    println!("Result {:?}", s.get(5..));
    

    s.clear();
}

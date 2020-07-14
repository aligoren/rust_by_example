fn main() {

    // Source: https://doc.rust-lang.org/book/ch03-02-data-types.html

    // we're converting string value to unsigned integer

    let _my_age: u32 = "27".parse().expect("Not a number!");
    
    println!("I'm {} years old!", _my_age);

    /* SCALAR TYPES */

    /*
        Integer Types

        Length          Signed          Unsigned
        ----------------------------------------
        8-bit           i8              u8
        16-bit          i16             u16
        32-bit          i32             u32
        64-bit          i64             u64
        128-bit         i128            u128
        arch            isize           usize

        For example 8-bit:
            Signed: from -(2^(n-1)) to 2^(n-1) - 1 => -128 tp 127
            Unsigned: from 0 to 2^(n-1) => 0 to 255
    */

    let _8bit_number: i8 = i8::MAX;

    println!("{} is 8 bit number", _8bit_number);

    let _16bit_number: u16 = u16::MAX;

    println!("{} is 16 bit number", _16bit_number);

    let _32bit_number: i32 = i32::MAX;

    println!("{} is 32 bit number", _32bit_number);

    let _64bit_number: i64 = i64::MAX;

    println!("{} is 64 bit number", _64bit_number);

    let _128bit_number: i128 = i128::MAX;

    println!("{} is 128 bit number", _128bit_number);

    let _isize_number: isize = isize::MAX;

    println!("{} is isize number", _isize_number);

    let _usize_number: usize = usize::MAX;

    println!("{} is usize number", _usize_number);

    /*
        Floting-Point Types._128bit_number

        By default, float numbers are f64
    */

    let _32_bit_floating: f32 = f32::MAX;

    println!("{} is 32-bit float number", _32_bit_floating);

    let _64_bit_floating: f64 = f64::MAX;

    println!("{} is 64-bit float number", _64_bit_floating);

    /*
        Numeric operations
    */

    let sum = 5 + 10;

    println!("Sum result: {}", sum);

    let diff = 20 - 5;

    println!("Difference result: {}", diff);

    let multiplication = 5 * 4;

    println!("Multiplication result: {}", multiplication);

    let division = 20 / 5;

    println!("Division result: {}", division);

    let remainder = 20 % 6;

    println!("Remainder result: {}", remainder);

    /*
        Boolean Type
    */

    let is_domain_checked: bool = true;

    println!("Is domain checked: {}", is_domain_checked);

    /*
        Character Type
    */

    let character: char = '🐈';

    println!("Selected emoji: {}", character);

    /*
        COMPOUND TYPES
    */

    /*
        Tuple Type
    */

    let person: (i32, &str, &str, bool) = (27, "Ali", "Software Developer", true);

    println!("Hi. I'm {}. I'm a {}. I'm {} years old! Did I started to learn Rust? Answer: {}", person.1, person.2, person.0, person.3);

    /*
        Array Type
    */

    let students = ["Ali", "Burak", "Talha"];

    println!("Students: {:?}", students);

    let _get_first_student = students[0];

    println!("First student's name is: {}", _get_first_student);

}

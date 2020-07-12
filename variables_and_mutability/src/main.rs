fn main() {
    
    // immutable variables by default. they cannot be changed until they aren't defined with mut keyword.
    let currentyear = 2020;
    let birth_year = 1993;

    // mutable variable. beucase it defined with mut keyword
    let mut age = currentyear - birth_year - 1;

    println!("I'm {}!", age);

    println!("No. This is wrong!");
    
    age = currentyear - birth_year;
    
    println!("I'm {}!", age);

    println!("Yep! That's true :P");

    // constant. when you define a constant variable, it cannot be changed and cannot be defined with a mut keyword.
    const PI: f64 = 3.14;

    println!("Pi is: {}", PI);
}

fn main() {
    
    // stack allocated integer variable
    let stack_allocated_x = 10;

    // copying variable x to y. not moving.
    let stack_allocated_y = stack_allocated_x;

    println!("X {} and Y {}", stack_allocated_x, stack_allocated_y);

    // there is no error. because these are primitive types.

    //heap allocated variable
    let mut heap_allocated_x = Box::new(10);

    let heap_allocated_y = heap_allocated_x;

    // won't work. because x's ownership moved to y
    //println!("X {} and Y {}", heap_allocated_x, heap_allocated_y);

    heap_allocated_x = box_example(heap_allocated_y);

    println!("X {}", heap_allocated_x);

    // won't work. because ownership is moved to the function
    // println!("Y {}", heap_allocated_y);

}


fn box_example(n: Box<u8>) -> Box<u8> {
    println!("N {}", n);

    n
}
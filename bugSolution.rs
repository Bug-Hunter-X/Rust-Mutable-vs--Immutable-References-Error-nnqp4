fn main() {
    let mut x = 5;
    let y = &mut x; // y is a mutable reference to x

    *y += 1; // Modify x through y
    println!("x = {}", x); // x is now 6
    println!("y = {}", *y); // y points to x, so *y is 6

    let z = &x; // z is an immutable reference to x
    println!("z = {}", *z); // z is 6
} 
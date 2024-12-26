fn main() {
    let mut x = 5;
    let y = &mut x; // y is a mutable reference to x
    let z = &x; // z is an immutable reference to x

    *y += 1; // Modify x through y
    println!("x = {}", x); // x is now 6
    println!("y = {}", *y); // y points to x, so *y is 6

    //This line will cause error because we are trying to modify x through z which is immutable
    //*z += 1; //Error! 
    println!("z = {}", *z); // z is 6
}
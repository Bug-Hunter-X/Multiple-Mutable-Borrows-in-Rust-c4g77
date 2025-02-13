fn main() {
    let mut x = 5;
    let y = &mut x; // y is a mutable reference to x
    let z = &mut x; // z is ALSO a mutable reference to x

    *y += 1; // Modifying x through y
    *z += 1; // ERROR! Modifying x through z causes a panic at runtime. 
}
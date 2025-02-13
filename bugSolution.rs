fn main() {
    let mut x = 5;

    // Solution 1: Use a single mutable reference.
    { 
        let y = &mut x;
        *y += 1;
    }
    println!("x after first modification: {}", x);

    // Solution 2:  Clone the data if modification is needed in multiple places
    let mut z = x.clone();
    z += 1;
    println!("z after cloning and modification: {}", z);
    println!("x remains unchanged: {}", x);

    // Solution 3: Using interior mutability with RefCell or Mutex (more advanced)

    use std::cell::RefCell;

    let x = RefCell::new(5);
    *x.borrow_mut() += 1;
    *x.borrow_mut() += 1;
    println!("x after modification with RefCell: {}", x.borrow());
}

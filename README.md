# Multiple Mutable Borrows in Rust
This repository demonstrates a common error in Rust: creating multiple mutable references to the same variable. Rust's ownership system prevents data races by disallowing this unless special care is taken. Attempting to create multiple mutable references to the same variable typically results in a compile-time error or a runtime panic.

The `bug.rs` file shows the problematic code, while `bugSolution.rs` shows ways to resolve it.

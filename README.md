# Data Race in Rust Using Raw Pointers

This repository demonstrates a potential data race in Rust that can occur when using raw pointers without proper synchronization.  The code modifies a vector directly via its raw pointer, which can lead to undefined behavior if accessed concurrently from another thread.

## Bug Description
The `bug.rs` file contains code that modifies a vector using an unsafe block and a raw pointer. If another thread attempts to access or modify the same vector simultaneously, this will result in a data race.

## Solution
The `bugSolution.rs` file demonstrates a safe alternative using proper synchronization mechanisms to prevent data races.
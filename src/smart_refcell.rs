use std::cell::RefCell;
// Using RefCell<T> to provide interior mutability
fn interior_mutability() {
    let data = RefCell::new(42);
    {
        let mut data_ref_mut = data.borrow_mut();
        *data_ref_mut += 1;
    }
    let data_ref = data.borrow();
    println!("Data: {}", data_ref);
}
// Runtime borrow checking and the potential for panics
fn potential_panic() {
    let data = RefCell::new(42);
    let data_ref = data.borrow();
    let mut data_ref_mut = data.borrow_mut(); // This will panic!
    println!("Data: {}", data_ref);
}

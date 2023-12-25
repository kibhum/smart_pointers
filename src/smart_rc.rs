use std::rc::Rc;
// Shared ownership and reference counting
fn shared_and_reference_counting() {
    let data = Rc::new("Hello, world!");
    let data_clone1 = Rc::clone(&data);
    let data_clone2 = Rc::clone(&data);
    println!("Original: {}", data);
    println!("Clone 1: {}", data_clone1);
    println!("Clone 2: {}", data_clone2);
    println!("Reference count (before): {}", Rc::strong_count(&data));
    take_ownership(data_clone2);
    println!("Reference count (after): {}", Rc::strong_count(&data));
}
fn take_ownership(data: Rc<&str>) {
    println!("Data in function: {}", data);
}
// Creating a tree-like data structure
struct Node {
    value: i32,
    next: Option<Rc<Node>>,
}
fn main() {
    let node1 = Rc::new(Node {
        value: 1,
        next: None,
    });
    let node2 = Rc::new(Node {
        value: 2,
        next: Some(Rc::clone(&node1)),
    });
    let node3 = Rc::new(Node {
        value: 3,
        next: Some(Rc::clone(&node2)),
    });
    println!("Node 1: {:?}", node1);
    println!("Node 2: {:?}", node2);
    println!("Node 3: {:?}", node3);
}

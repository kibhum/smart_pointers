// Making enum representations more uniform
enum Sizes {
    S,
    M,
    L,
    XL(Box<[u32; 1024]>),
    XXL(Box<[u32; 1024 * 1024]>),
}

fn smart_box() {
    // Allocate a large array on the heap
    let data = Box::new([0; 1024 * 1024]);
    // Print out its length
    println!("Length: {}", data.len());
}

// Recursive data structures
#[derive(Debug)]
struct List(Option<Box<Item>>);
#[derive(Debug)]
struct Item(i32, Option<Box<Item>>);

impl List {
    fn append(&mut self, value: i32) {
        let mut current: &mut Option<Box<Item>> = &mut self.0;
        while let Some(ref mut next_item) = current {
            current = &mut next_item.1;
        }
        let item: Item = Item(value, None);
        *current = Some(Box::new(item));
    }
}

fn main() {
    let mut l = List(None);
    l.append(1);
    l.append(2);
    println!("{l:?}")
}

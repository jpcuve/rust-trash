#[derive(Debug)]
struct Container {
    a_items: Vec<ItemA>,
    b_items: Vec<ItemB>
}

impl Container {
    fn new() -> Self {
        Container {
            a_items: vec!{},
            b_items: vec!{},
        }
    }
}

#[derive(Debug)]
struct Item {
    name: String,
}

impl Item {
    fn new(name: String) -> Self {
        Item { name }
    }
}

#[derive(Debug)]
struct ItemA {
    item: Item,
}

#[derive(Debug)]
struct ItemB {
    item: Item,
}

fn main() {
    println!("Hello, world!");
    let mut c = Container::new();
    println!("{:?}", c);
}

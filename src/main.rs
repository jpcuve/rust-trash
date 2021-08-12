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

#[derive(Debug)]
enum Phase {
    Intro,
    Game,
    Credits,
}

trait Update{
    fn update(&self) -> Self;
}

impl Update for Phase {
    fn update(&self) -> Self {
        match &self {
            Phase::Intro => Phase::Game,
            Phase::Game => Phase::Credits,
            Phase::Credits => Phase::Intro
        }
    }
}

fn main() {
    println!("Hello, world!");
    let c = Container::new();
    println!("{:?}", c);
    let mut phase = Phase::Intro;
    for _ in 0..10 {
        phase = phase.update();
        println!("{:?}", phase);
    }
}

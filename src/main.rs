mod interface;
mod game_design;
mod state_machine;

use std::error::Error;
use crate::interface::inter;
use crate::game_design::Game;
use crate::state_machine::State;


#[derive(Debug)]
struct Container {
    a_items: Vec<ItemA>,
    b_items: Vec<ItemB>
}

impl Default for Container {
    fn default() -> Self {
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
    fn new(name: &str) -> Self {
        Item { name: String::from(name) }
    }
}

#[derive(Debug)]
struct ItemA {
    item: Item,
}

impl ItemA {
    fn something(&self){
        println!("Item A!");
    }
}

#[derive(Debug)]
struct ItemB {
    item: Item,
}

#[derive(Debug)]
enum Phase {
    Intro(ItemA),
    Game(ItemB),
    Credits(Item),
}

fn test(){
    println!("Hello, world!");
    let c = Container::default();
    println!("{:?}", c);
    let item_a = ItemA { item: Item::new("Item A")};
    let item_b = ItemB { item: Item::new("Item B")};
    let item_c = Item::new("Item");
    let phase_a = Phase::Intro(item_a);
    let phase_b = Phase::Game(item_b);
    let phase_c = Phase::Credits(item_c);
    let mut phase = &phase_a;
    for _ in 0..10 {
        let phase = match phase {
            Phase::Intro(item) => {
                item.something();
                &phase_b
            },
            Phase::Game(_) => &phase_c,
            Phase::Credits(_) => &phase_a,
        };
        // phase = phase.update();
        println!("{:?}", phase);
    }
}

fn main() {
    let mut state = State::StateA;
    state = state.transition();
    println!("{:?}", state);
/*
    let game = Game::default();
    game.update();
*/
}

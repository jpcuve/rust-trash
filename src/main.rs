mod interface;
mod game_design;
mod state_machine;
mod lisp;

use std::convert::Infallible;
use std::error::Error;
use std::net::SocketAddr;
use hyper::{Body, Request, Response, Server};
use hyper::service::{make_service_fn, service_fn};
use crate::interface::inter;
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

async fn hello_world(_req: Request<Body>) -> Result<Response<Body>, Infallible>{
    Ok(Response::new("Hello, world".into()))
}

#[tokio::main]
async fn main() {
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    let make_svc = make_service_fn(|_conn| async {
        Ok::<_, Infallible>(service_fn(hello_world))
    });

    let server = Server::bind(&addr).serve(make_svc);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}
use nalgebra_glm::Vec3;
use rand::prelude::*;

#[derive(Default, Debug)]
struct Shape {

}

#[derive(Default, Debug)]
struct Spatial {
    position: Vec3,
    speed: Vec3,
    acceleration: Vec3,
}

#[derive(Default, Debug)]
struct GameObject {
    shape: Shape,
    spatial: Spatial,
    state: u32,
}

impl GameObject {
    fn hits(&self, other: &GameObject) -> bool {
        rand::random::<bool>()
    }
}

#[derive(Default, Debug)]
struct Ship {
    game_object: GameObject,
}

impl Ship {
    fn thrust(&self){}
    fn rotate(&self, clockwise: bool){}
    fn fire(&self, bullets: &Vec<Bullet>){}
    fn collide_asteroids(&self, asteroids: &mut Vec<Asteroid>){
        asteroids.retain(|a| !a.game_object.hits(&self.game_object))
    }
}

#[derive(Default, Debug)]
struct Bullet {
    game_object: GameObject,
}

impl Bullet {
    fn collide_asteroids(&self, asteroids: &mut Vec<Asteroid>){
        asteroids.retain(|a| !a.game_object.hits(&self.game_object))
    }
}

#[derive(Default, Debug)]
struct Asteroid {
    game_object: GameObject,
}

trait Screen {
    fn update(&self);
    fn paint(&self);
}

#[derive(Default, Debug)]
struct Game {
    ships: Vec<Ship>,
    bullets: Vec<Bullet>,
    asteroids: Vec<Asteroid>
}

impl Screen for Game {
    fn update(&self){
        for ship in self.ships.iter(){
            ship.fire(&self.bullets)
        }
    }

    fn paint(&self) {
        todo!()
    }
}

#[derive(Default, Debug)]
struct Introduction;

impl Screen for Introduction {
    fn update(&self) {
        todo!()
    }

    fn paint(&self) {
        todo!()
    }
}

#[derive(Default, Debug)]
struct Credits;

impl Screen for Credits {
    fn update(&self) {
        todo!()
    }

    fn paint(&self) {
        todo!()
    }
}

pub fn run_game(){
    let game = Box::new(Game::default());
    let introduction = Box::new(Introduction::default());
    let credits = Box::new(Credits::default());
    let mut screen: Box<dyn Screen> = game;
    screen = introduction;
    screen.update();
    screen = credits;
}

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
struct Action {
    ships: Vec<Ship>,
    bullets: Vec<Bullet>,
    asteroids: Vec<Asteroid>
}

impl Screen for Action {
    fn update(&self){
        for ship in self.ships.iter(){
            ship.fire(&self.bullets)
        }
    }

    fn paint(&self) {
        todo!()
    }
}

struct Introduction;
struct Credits;

pub struct Game {
    screen: Box<dyn Screen>,
}

impl Default for Game {
    fn default() -> Self {
        Game{screen: Box::new(Action::default())}
    }
}

impl Game {
    pub fn update(&self){
        self.screen.update();
    }
}
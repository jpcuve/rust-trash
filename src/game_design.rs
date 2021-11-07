use rand::prelude::*;

#[derive(Default, Debug)]
struct Shape {

}

#[derive(Default, Debug)]
struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
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

#[derive(Default, Debug)]
pub struct Game {
    ships: Vec<Ship>,
    bullets: Vec<Bullet>,
    asteroids: Vec<Asteroid>
}

impl Game {
    pub fn update(&self){
        for ship in self.ships.iter(){
            ship.fire(&self.bullets)
        }
    }
}

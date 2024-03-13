#![allow(unused)]
use std::io;
#[derive(Debug)]
enum State {
    Mario,
    SuperMario,
    FireMario,
    CapeMario
}
use State::*;
#[derive(Debug)]
enum Transition {
    Mushroom,
    Feather,
    Flower,
}
use Transition::*;

struct Player {
    state: State
}
impl Player {
    fn new() -> Player  {
        Player {
            state: Mario
        }
    }
    fn collect(& mut self, power: Transition) {
        match (&self.state, power) {
            (Mario, Mushroom) => self.state = SuperMario,
            (_,     Feather)  => self.state = FireMario,
            (_,     Flower)   => self.state = CapeMario,
            (_,     Mushroom) => {}
        }
    }
}

fn main() {
    let mut player = Player::new();
    player.collect(Mushroom);
    player.collect(Feather);
    player.collect(Flower);
    player.collect(Mushroom);
    //player.collect(Mushroom);
    println!("last state: {:?}", player.state);
}

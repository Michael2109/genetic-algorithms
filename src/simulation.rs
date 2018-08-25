extern crate rand;

use std::cell::Cell;
use rand::Rng;

use vector2d::Vector2D;
use entity::{Entity, Fish, Food};

pub struct Simulation {
    pub fish: Vec<Fish>,
    pub food: Vec<Food>
}

pub trait SimulationTrait {
    fn initialize(&mut self);

    fn update(&mut self);

    fn create_fish(&mut self) -> Fish;
    fn create_food(&mut self) -> Food;

    fn get_random_position(&mut self) -> Vector2D;
}

impl SimulationTrait for Simulation {

    fn initialize(&mut self) {

        let fish = self.create_fish();
        let food = self.create_food();

        self.fish.push(fish);
        self.food.push(food);

        for _ in 0..50 {
            let fish = self.create_fish();
            self.fish.push(fish);
        }

        for _ in 0..10 {
            let food = self.create_food();
            self.food.push(food);
        }
    }

    fn update(&mut self) {
        for mut f in &self.fish {
            f.update(&self);
        }
    }

    fn create_fish(&mut self) -> Fish {
        Fish {
            acceleration: 1.0,
            target_mass: 1.0,
            range: 100.0,
            multiply_time: Cell::new(1.0),

            position: self.get_random_position(),
            dx: Cell::new(0.0),
            dy: Cell::new(0.0),
            rotation: Cell::new(0.0),
            energy: Cell::new(1.0),
            alive: Cell::new(true)
        }
    }

    fn create_food(&mut self) -> Food {
        Food {
            energy: 100.0,
            position: self.get_random_position(),
            alive: Cell::new(true)
        }
    }

    fn get_random_position(&mut self) -> Vector2D {
        Vector2D {
            x: Cell::new(rand::thread_rng().gen_range(0.0, 1000.0)),
            y: Cell::new(rand::thread_rng().gen_range(0.0, 1000.0))
        }
    }
}

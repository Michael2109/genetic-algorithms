use std::cell::Cell;

use entity::{Entity, Fish, Food};

pub struct Simulation {
    pub fish: Vec<Fish>,
    pub food: Vec<Food>
}

pub trait SimulationTrait {
    fn initialize(&mut self);

    fn update(&mut self);

    fn createFish(&mut self) -> Fish;
    fn createFood(&mut self) -> Food;


}

impl SimulationTrait for Simulation {

    fn initialize(&mut self) {

        let fish = Fish {
            acceleration: 1.0,
            target_mass: 1.0,
            range: 100.0,
            multiply_time: Cell::new(1.0),

            x: Cell::new(10.0),
            y: Cell::new(10.0),
            dx: Cell::new(0.0),
            dy: Cell::new(0.0),
            rotation: Cell::new(0.0),
            energy: Cell::new(1.0)
        };

        let food = Food {
            energy: 100.0,
            x: Cell::new(500.0),
            y: Cell::new(500.0),
        };

        self.fish.push(fish);
        self.food.push(food);
    }

    fn update(&mut self) {
        for mut f in &self.fish {
            f.update(&self.food);
        }
    }

    fn createFish(&mut self) -> Fish {

    }

    fn createFood(&mut self) -> Food {

    }
}

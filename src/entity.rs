use std::cell::Cell;

use point::Point2D;
use simulation::Simulation;

pub struct Fish {
    pub acceleration: f64,
    pub target_mass: f64,
    pub range: f64,
    pub multiply_time: Cell<f64>,

    pub position: Point2D,
    pub dx: Cell<f64>,
    pub dy: Cell<f64>,
    pub rotation: Cell<f64>,
    pub energy: Cell<f64>
}

pub struct Food {
    pub energy: f64,
    pub position: Point2D
}

pub trait Entity {
    fn update(&self, simulation: &Simulation);
    fn get_target(&self, simulation: &Simulation) -> Point2D;
}

impl Entity for Fish {

    fn update(&self, simulation: &Simulation) {
        self.position.x.set(self.position.x.get() + 1.0);

        let target_pos = self.get_target(simulation);

        let distance = ((self.position.x.get() - target_pos.x.get()).powf(2.0) + (self.position.y.get() - target_pos.y.get()).powf(2.0)).sqrt();

    }

    fn get_target(&self, simulation: &Simulation) -> Point2D {
        //let in_range = food.into_iter().filter(|f| ((self.x.get() - f.x.get()).powf(2.0) + (self.y.get() - f.y.get()).powf(2.0)).sqrt() < self.range).collect()[0];
        //(in_range.x.get(), in_range.y.get())
        Point2D {
            x: Cell::new(simulation.food[0].position.x.get()),
            y: Cell::new(simulation.food[0].position.y.get())
        }
    }
}

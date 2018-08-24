use std::cell::Cell;

use point::Point2D;

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
    fn update(&self, food: &Vec<Food>);
    fn get_target(&self, food: &Vec<Food>) -> (f64, f64);
}

impl Entity for Fish {

    fn update(&self, food: &Vec<Food>) {
        self.position.x.set(self.position.x.get() + 1.0);

        let targetPos = self.get_target(food);

    }

    fn get_target(&self, food: &Vec<Food>) -> (f64, f64) {
        //let in_range = food.into_iter().filter(|f| ((self.x.get() - f.x.get()).powf(2.0) + (self.y.get() - f.y.get()).powf(2.0)).sqrt() < self.range).collect()[0];
        //(in_range.x.get(), in_range.y.get())
        (food[0].position.x.get(), food[0].position.y.get())
    }
}

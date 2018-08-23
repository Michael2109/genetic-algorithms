extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

use piston::input::*;
use std::cell::Cell;

pub struct Fish {
    pub acceleration: f64,
    pub target_mass: f64,
    pub range: f64,
    pub multiply_time: Cell<f64>,

    pub x: Cell<f64>,
    pub y: Cell<f64>,
    pub dx: Cell<f64>,
    pub dy: Cell<f64>,
    pub rotation: Cell<f64>,
    pub energy: Cell<f64>
}

pub struct Food {
    pub energy: f64,
    pub x: Cell<f64>,
    pub y: Cell<f64>
}

pub trait Entity {
    fn update(&self, args: &UpdateArgs, food: Vec<Food>);
    fn getTarget(&self, food: Vec<Food>) -> (i64, i64);
}

impl Entity for Fish {

    fn update(&self, args: &UpdateArgs, food: Vec<Food>) {
        self.x.set(self.x.get() + 1.0);

    }

    fn getTarget(&self, food: Vec<Food>) -> (i64, i64) {
        let in_range = food.into_iter().filter(|f| ((self.x.get() - f.x.get()).powf(2.0) + (self.y.get() - f.y.get()).powf(2.0)).sqrt() < self.range).collect()[0];
        (in_range.x.get(), in_range.y.get())
    }
}

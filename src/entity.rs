extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

use piston::input::*;
use std::cell::Cell;

pub struct Fish {
    pub mass: f64,
    pub fish_food_ratio: f64,
    pub target_mass: f64,
    pub target_mass_range: f64,

    pub x: Cell<f64>,
    pub y: Cell<f64>,
    pub hunger: Cell<f64>,
    pub multiply_time: Cell<f64>,
    pub rotation: Cell<f64>,
}

pub struct Food {
    pub mass: f64,

    pub x: Cell<f64>,
    pub y: Cell<f64>,
}

pub trait Entity {
    fn update(&self, args: &UpdateArgs);
}

impl Entity for Fish {

    fn update(&self, args: &UpdateArgs) {
        self.x.set(self.x.get() + 1.0)
    }
}

impl Entity for Food {

    fn update(&self, args: &UpdateArgs) {
    }
}

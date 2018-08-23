extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics };
use std::cell::Cell;

pub fn print_hello() {
    println!("Hello, world!");
}

pub struct Fish {
    pub gl: GlGraphics, // OpenGL drawing backend.

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
    pub gl: GlGraphics, // OpenGL drawing backend.

    pub mass: f64,

    pub x: Cell<f64>,
    pub y: Cell<f64>,
}

pub trait Entity {
    fn update(mut self, args: &UpdateArgs);
}

impl Entity for Fish {

    fn update(mut self, _args: &UpdateArgs) {
    }
}

impl Entity for Food {

    fn update(mut self, _args: &UpdateArgs) {
    }
}

extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL };
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
    fn render(mut self, args: &RenderArgs);
}

impl Entity for Fish {

    fn update(mut self, _args: &UpdateArgs) {
    }

    fn render(mut self, _args: &RenderArgs) {
        use graphics::*;

        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
        const RED:   [f32; 4] = [1.0, 0.0, 0.0, 1.0];

        let square = rectangle::square(0.0, 0.0, 50.0);
        let rotation = self.rotation;
        let (x, y) = (self.x.get(), self.y.get());

        self.gl.draw(_args.viewport(), |c, gl| {
            // Clear the screen.
            clear(GREEN, gl);

            let transform = c.transform.trans(x, y)
                .rot_rad(rotation.get())
                .trans(-25.0, -25.0);

            // Draw a box rotating around the middle of the screen.
            rectangle(RED, square, transform, gl);
        });
    }
}

impl Entity for Food {

    fn update(mut self, _args: &UpdateArgs) {
    }

    fn render(mut self, _args: &RenderArgs) {
        use graphics::*;

        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
        const RED:   [f32; 4] = [1.0, 0.0, 0.0, 1.0];

        let square = rectangle::square(0.0, 0.0, 50.0);

        let (x, y) = ((_args.width / 2) as f64,
                      (_args.height / 2) as f64);

        self.gl.draw(_args.viewport(), |c, gl| {
            // Clear the screen.
            clear(GREEN, gl);

            let transform = c.transform.trans(x, y)
                .trans(-25.0, -25.0);

            // Draw a box rotating around the middle of the screen.
            rectangle(RED, square, transform, gl);
        });
    }
}

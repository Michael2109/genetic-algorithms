mod entity;

extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use std::cell::Cell;
use entity::{Entity, Fish, Food};

pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    fish: Vec<Fish>,
    food: Vec<Food>
}

impl App {
    fn update(&mut self, args: &UpdateArgs) {
        for mut f in &self.fish {
            f.update(args, self.food);
        }
    }
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
        const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];

        let fish_rectangle = rectangle::square(0.0, 0.0, 50.0);
        let food_rectangle = rectangle::square(0.0, 0.0, 20.0);

        self.gl.draw(args.viewport(), |_, gl| {
            clear(BLACK, gl);
        });

        for f in &self.fish {
            self.gl.draw(args.viewport(), |c, gl| {

                let transform = c.transform.trans(f.x.get(), f.y.get())
                    .rot_rad(f.rotation.get())
                    .trans(-25.0, -25.0);

                // Draw a box rotating around the middle of the screen.
                ellipse(RED, fish_rectangle, transform, gl);
            });
        }

        for f in &self.food {
            self.gl.draw(args.viewport(), |c, gl| {

                let transform = c.transform.trans(f.x.get(), f.y.get())
                    .trans(-10.0, -10.0);

                // Draw a box rotating around the middle of the screen.
                ellipse(GREEN, food_rectangle, transform, gl);
            });
        }
    }
}

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create an Glutin window.
    let mut window: Window = WindowSettings::new(
        "spinning-square",
        [800, 600],
    )
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    // Create a new game and run it.
    let mut app = App {
        gl: GlGraphics::new(opengl),
        fish: Vec::new(),
        food: Vec::new()
    };

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

    app.fish.push(fish);
    app.food.push(food);

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(u) = e.update_args() {
            app.update(&u);
        }
        if let Some(r) = e.render_args() {
            app.render(&r);
        }

    }
}

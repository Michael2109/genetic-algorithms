mod entity;
mod simulation;
mod point;

extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use simulation::{Simulation, SimulationTrait};

pub struct App {
    gl: GlGraphics,
    simulation: Simulation
}

impl App {

    fn initialize(&mut self) {
        self.simulation.initialize();
    }

    fn update(&mut self) {
        self.simulation.update();
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

        for f in &self.simulation.fish {
            self.gl.draw(args.viewport(), |c, gl| {

                let transform = c.transform.trans(f.position.x.get(), f.position.y.get())
                    .rot_rad(f.rotation.get())
                    .trans(-25.0, -25.0);

                // Draw a box rotating around the middle of the screen.
                ellipse(RED, fish_rectangle, transform, gl);
            });
        }

        for f in &self.simulation.food {
            self.gl.draw(args.viewport(), |c, gl| {

                let transform = c.transform.trans(f.position.x.get(), f.position.y.get())
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
        simulation: Simulation {
            fish: Vec::new(),
            food: Vec::new()
        }
    };

    app.initialize();

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(u) = e.update_args() {
            app.update();
        }
        if let Some(r) = e.render_args() {
            app.render(&r);
        }

    }
}

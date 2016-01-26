//#[macro_use]
//extern crate glium;
extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;
extern crate rand;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL };
use std::thread;
use std::sync::{Arc, Mutex};

mod cell;
mod grid;
use grid::Grid;


const WINDOW_WIDTH: u32 = 720;
const WINDOW_HEIGHT: u32  = 720;
const CELL_SIZE: u32 = 12; // must be a divisor of width and height and 6
const UPDATES_PER_SECOND: u32 = 30;

pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    grid: Arc<Mutex<Grid>>  // Cell Grid
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const BLUE: [f32; 4] = [0.0, 0.0, 1.0, 1.0];
        const LIGHT_BLUE: [f32; 4] = [0.4, 0.4, 0.9, 1.0];
        const WHITE:   [f32; 4] = [1.0, 1.0, 1.0, 1.0];

        let square = rectangle::square(0.0, 0.0, CELL_SIZE as f64);
        let child_square = rectangle::square((CELL_SIZE / 6) as f64, (CELL_SIZE / 6) as f64, (CELL_SIZE - (CELL_SIZE / 3)) as f64);

        let alive_cells = self.grid.lock().unwrap().alive_cells();
        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(WHITE, gl);

            // Draw all of the cells
            for cell in alive_cells {
                let transform = c.transform.trans((cell.x * CELL_SIZE) as f64, (cell.y * CELL_SIZE) as f64);

                rectangle(BLUE, square, transform, gl);
                rectangle(LIGHT_BLUE, child_square, transform, gl);
            }
            ()
        });
    }

    fn update(&mut self, args: &UpdateArgs) {
    }
}

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;
    // Create an Glutin window.
    let mut window: Window = WindowSettings::new(
            "Conway's Game of Life",
            [WINDOW_WIDTH, WINDOW_HEIGHT]
        )
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();


    let cell_grid = Arc::new(Mutex::new(Grid::new((WINDOW_WIDTH/CELL_SIZE) as u32, (WINDOW_HEIGHT/CELL_SIZE) as u32)));
    // Spawn thread to update cell;
    {
        let grid = cell_grid.clone();
        thread::spawn(move || {
            loop {
                thread::sleep_ms(1000 / UPDATES_PER_SECOND);
                grid.lock().unwrap().tick();
            }
        });
    }


    // Create a new game and run it.
    let mut app = App {
        gl: GlGraphics::new(opengl),
        grid: cell_grid
    };

    let mut events = window.events();
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            app.render(&r);
        }
        if let Some(u) = e.update_args() {
            app.update(&u);
        }
    }
}

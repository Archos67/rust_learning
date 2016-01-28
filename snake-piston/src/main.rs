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

mod snake;
use snake::Snake;
use snake::Block;

// named constants
const WINDOW_WIDTH: u32 = 720;
const WINDOW_HEIGHT: u32  = 720;
const BLOCK_SIZE: u32 = 12; // must be a divisor of width and height and 6
const UPDATES_PER_SECOND: u32 = 5;


pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    snake: Arc<Mutex<Snake>>
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const BLUE: [f32; 4] = [0.0, 0.0, 1.0, 1.0];
        const LIGHT_BLUE: [f32; 4] = [0.4, 0.4, 0.9, 1.0];
        const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
        const LIGHT_RED: [f32; 4] = [1.0, 0.4, 0.4, 1.0];
        const WHITE:   [f32; 4] = [1.0, 1.0, 1.0, 1.0];

        let square = rectangle::square(0.0, 0.0, BLOCK_SIZE as f64);
        let child_square = rectangle::square((BLOCK_SIZE / 6) as f64, (BLOCK_SIZE / 6) as f64, (BLOCK_SIZE - (BLOCK_SIZE / 3)) as f64);

        let snake = self.snake.lock().unwrap();
        let blocks = snake.snake.clone();
        let candy = snake.candy.clone();

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(WHITE, gl);

            // Draw all of the cells
            for block in blocks {
                let transform = c.transform.trans((block.x * BLOCK_SIZE) as f64, (block.y * BLOCK_SIZE) as f64);

                rectangle(BLUE, square, transform, gl);
                rectangle(LIGHT_BLUE, child_square, transform, gl);
            }
            let transform = c.transform.trans((candy.x * BLOCK_SIZE) as f64, (candy.y * BLOCK_SIZE) as f64);
            rectangle(RED, square, transform, gl);
            rectangle(LIGHT_RED, child_square, transform, gl);
        });
    }

    fn update(&mut self, args: &UpdateArgs) {

    }

    fn key_press(&mut self, key: Key) {
        use piston::input::keyboard::Key::*;
        match key {
            Up | Down | Left | Right => {
                self.snake.lock().unwrap().change_direction(key);
            },
            _ => {
            }
        };
    }
}

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;
    // Create an Glutin window.
    let mut window: Window = WindowSettings::new(
            "Snake Rust",
            [WINDOW_WIDTH, WINDOW_HEIGHT]
        )
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();
    let snake = Arc::new(Mutex::new(Snake::new((WINDOW_WIDTH/BLOCK_SIZE) as u32, (WINDOW_HEIGHT/BLOCK_SIZE) as u32)));
    // Spawn thread to update snake
    {
        let snake = snake.clone();
        thread::spawn(move || {
            loop {
                thread::sleep_ms(1000 / UPDATES_PER_SECOND);
                let mut s = snake.lock().unwrap();
                if s.alive {
                    s.tick();
                }
            }
        });
    }
    // Create a new game and run it.
    let mut app = App {
        gl: GlGraphics::new(opengl),
        snake: snake
    };
    let mut events = window.events();
    while let Some(e) = events.next(&mut window) {
        match e {
            Event::Render(args) => {
                app.render(&args);
            }

            Event::Input(Input::Press(Button::Keyboard(key))) => {
                app.key_press(key);
            }

            Event::Update(args) => {
                app.update(&args)
            }

            _ => {}
        }
    }
/*
    let mut events = window.events();
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            app.render(&r);
        }
        if let Some(u) = e.update_args() {
            app.update(&u);
        }
        if let Some(u) = e.key_press() {
            app.keypress
        }
    }
    */
}

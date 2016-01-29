extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;
extern crate image;
extern crate rand;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL };
use opengl_graphics::glyph_cache::GlyphCache;
use std::thread;
use std::sync::{Arc, Mutex};
use std::path::Path;
use std::fs::OpenOptions;


mod snake;
use snake::Snake;

// named constants
const WINDOW_WIDTH: u32 = 720;
const WINDOW_HEIGHT: u32  = 720;
const BLOCK_SIZE: u32 = 12; // must be a divisor of width and height and 6
const UPDATES_PER_SECOND: u32 = 10;


pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    snake: Arc<Mutex<Snake>>,
    cache: GlyphCache<'static> // Thanks to tetris-piston for the example!!!
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const GREEN: [f32; 4] = [0.2, 0.8, 0.2, 1.0];
        const LIGHT_GREEN: [f32; 4] = [0.4, 0.9, 0.4, 1.0];
        const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
        const LIGHT_RED: [f32; 4] = [1.0, 0.4, 0.4, 1.0];
        const WHITE:   [f32; 4] = [1.0, 1.0, 1.0, 1.0];
        const TRANS_BLACK:   [f32; 4] = [0.0, 0.0, 0.0, 0.5];

        let square = rectangle::square(0.0, 0.0, BLOCK_SIZE as f64);
        let child_square = rectangle::square((BLOCK_SIZE / 6) as f64, (BLOCK_SIZE / 6) as f64, (BLOCK_SIZE - (BLOCK_SIZE / 3)) as f64);

        let snake = self.snake.lock().unwrap();
        let use_cache = &mut self.cache;
        let blocks = snake.snake.clone();
        let candy = snake.candy.clone();
        //t.draw("Hello");
        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(WHITE, gl);


            // Draw all of the blocks of the snake
            for block in blocks {
                let transform = c.transform.trans((block.x * BLOCK_SIZE) as f64, (block.y * BLOCK_SIZE) as f64);

                rectangle(GREEN, square, transform, gl);
                rectangle(LIGHT_GREEN, child_square, transform, gl);
            }
            // Draw Candy
            let transform = c.transform.trans((candy.x * BLOCK_SIZE) as f64, (candy.y * BLOCK_SIZE) as f64);
            rectangle(RED, square, transform, gl);
            rectangle(LIGHT_RED, child_square, transform, gl);

            // Draw score and high score
            let mut transform: graphics::context::Context = c.trans(50.0, 50.0);
            let mut t = Text::new(18);
            t.draw(&format!("High Score: {}", snake.high_score), use_cache, &transform.draw_state, transform.transform, gl);
            transform = c.trans(50.0, 25.0);
            t.draw(&format!("Score: {}", snake.current_score), use_cache, &transform.draw_state, transform.transform, gl);

            // Draw Pause overlay
            if snake.paused || !snake.alive {
                let transform = c.transform.trans(0.0, 0.0).scale(100.0, 100.0);
                rectangle(TRANS_BLACK, square, transform, gl );
                if snake.alive {
                    // Draw Paused
                    t.color = WHITE;
                    let mut transform = c.trans(300.0, 100.0);
                    t.draw("Paused", use_cache, &transform.draw_state, transform.transform, gl);
                    // Draw P to Unpause
                    transform = c.trans(300.0, 150.0);
                    t.draw("P to Unpause", use_cache, &transform.draw_state, transform.transform, gl);

                } else {
                    // Draw "Game Over"
                    t.color = WHITE;
                    let mut transform = c.trans(300.0, 100.0);
                    t.draw("Game Over", use_cache, &transform.draw_state, transform.transform, gl);

                    // Draw "R to Restart"
                    transform = c.trans(300.0, 150.0);
                    t.draw("R to Restart", use_cache, &transform.draw_state, transform.transform, gl);
                }
            }
        });
    }

    fn update(&mut self, args: &UpdateArgs) {
        //TODO What even happens here
    }

    fn key_press(&mut self, key: Key) {
        use piston::input::keyboard::Key::*;
        match key {
            Up | Down | Left | Right => {
                self.snake.lock().unwrap().change_direction(key);
            }
            P => {
                self.snake.lock().unwrap().pause();
            }
            R => {
                if !self.snake.lock().unwrap().alive{
                    self.snake.lock().unwrap().reset();
                }
            }
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
    // Load the fonts
    let font_path = match OpenOptions::new().open("OpenSans-Semibold.ttf") {
        Ok(_) => Path::new("FiraMono-Bold.ttf"),
        Err(_) => {
            match OpenOptions::new().open("src/OpenSans-Semibold.ttf") {
                Ok(_) => Path::new("src/OpenSans-Semibold.ttf"),
                Err(_) => panic!("Font file is missing, or does not exist in the current path."),
            }
        }
    };
    // Create an Arc<Mutex<Snake>> to wrap around the game to allow the renderer and the main thread to share resources
    let snake = Arc::new(Mutex::new(Snake::new((WINDOW_WIDTH/BLOCK_SIZE) as u32, (WINDOW_HEIGHT/BLOCK_SIZE) as u32)));
    // Spawn thread to update snake
    {
        let snake = snake.clone();
        thread::spawn(move || {
            loop {
                thread::sleep_ms(1000 / UPDATES_PER_SECOND);
                let mut s = snake.lock().unwrap();
                if s.alive && !s.paused {
                    s.tick();
                }
            }
        });
    }
    // Create a new game and run it.
    let mut app = App {
        gl: GlGraphics::new(opengl),
        snake: snake,
        cache: GlyphCache::new(font_path).unwrap(),
    };

    //Monitor events
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
}

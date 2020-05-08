mod food;
mod game;
mod helpers;
mod snake;

use std::io::Write;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::OpenGL;
use piston::event_loop::{EventLoop, EventSettings, Events};
use piston::input::{ButtonEvent, RenderEvent, UpdateEvent};
use piston::window::WindowSettings;

use game::Game;
use helpers::Status;

fn main() {
    let opengl = OpenGL::V3_2;

    const COLS: u32 = 40;
    const ROWS: u32 = 30;
    const SQUARE_WIDTH: u32 = 20;

    const SCREEN_WIDTH: u32 = COLS * SQUARE_WIDTH;
    const SCREEN_HEIGHT: u32 = ROWS * SQUARE_WIDTH;

    let mut window: Window = WindowSettings::new("Snake", [SCREEN_WIDTH, SCREEN_HEIGHT])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut game = Game::new(opengl, ROWS, COLS, SQUARE_WIDTH, 0);

    let mut events = Events::new(EventSettings::new());
    events.set_ups(10);

    while let Some(e) = events.next(&mut window) {
        if game.status == Status::NORMAL {
            if let Some(render_args) = e.render_args() {
                game.render(&render_args);
            }

            if let Some(_update_args) = e.update_args() {
                if !game.update() {
                    game.status = Status::OVER;
                    std::io::stdout()
                        .write_all(
                            b"\nGame Over! Press [Esc] to exit or [Space] to start over...\n",
                        )
                        .expect("Writing to stdout failed");
                }
            }
        }

        if let Some(button_args) = e.button_args() {
            if game.status == Status::OVER
                && button_args.button == piston::Button::Keyboard(piston::Key::Space)
            {
                println!("Restarting");
                let best_score = game.best_score;
                game = Game::new(opengl, ROWS, COLS, SQUARE_WIDTH, best_score);
            } else {
                game.handle_keypress(button_args.button);
            }
        }
    }

    println!("\nBest score: {}", game.best_score);
}

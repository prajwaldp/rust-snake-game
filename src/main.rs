mod game;
mod snake;
mod food;
mod helpers;

use std::collections::LinkedList;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events, EventLoop};
use piston::input::{RenderEvent, UpdateEvent, ButtonEvent};
use piston::window::WindowSettings;

use game::Game;
use snake::{Snake, SnakePiece};
use food::Food;
use helpers::Direction;

fn main() {
    let opengl = OpenGL::V3_2;

    const COLS: u32 = 30;
    const ROWS: u32 = 20;
    const SQUARE_WIDTH: u32 = 20;

    const SCREEN_WIDTH: u32 = COLS * SQUARE_WIDTH;
    const SCREEN_HEIGHT: u32 = ROWS * SQUARE_WIDTH;

    let mut window: Window = WindowSettings::new("Snake", [SCREEN_WIDTH, SCREEN_HEIGHT])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut snake_parts = LinkedList::new();
    snake_parts.push_back(SnakePiece(ROWS / 2, COLS / 2));

    let mut game = Game {
        gl: GlGraphics::new(opengl),
        rows: ROWS,
        cols: COLS,
        square_width: SQUARE_WIDTH,
        snake: Snake {
            gl: GlGraphics::new(opengl),
            parts: snake_parts,
            width:  SQUARE_WIDTH,
            d: Direction::DOWN
        },
        food: Food(1, 1),
        just_eaten: false
    };

    let mut events = Events::new(EventSettings::new());
    events.set_ups(10);
    while let Some(e) = events.next(&mut window) {
        if let Some(render_args) = e.render_args() {
            game.render(&render_args);
        }

        if let Some(update_args) = e.update_args() {
            game.update(&update_args);
        }

        if let Some(button_args) = e.button_args() {
            game.handle_keypress(button_args.button);
        }
    }
}

use opengl_graphics::{GlGraphics, OpenGL};
use piston::input::RenderArgs;
use rand::prelude::*;

use std::collections::LinkedList;
use std::io::Write;

use crate::food::Food;
use crate::helpers::{Direction, Status};
use crate::snake::{Snake, SnakePiece};

pub struct Game {
    pub gl: GlGraphics,
    pub rows: u32,
    pub cols: u32,
    pub square_width: u32,
    pub snake: Snake,
    pub food: Food,
    pub just_eaten: bool,
    pub score: u32,
    pub best_score: u32,
    pub status: Status,
}

impl Game {
    pub fn new(opengl: OpenGL, rows: u32, cols: u32, width: u32, best_score: u32) -> Game {
        let mut snake_parts = LinkedList::new();
        snake_parts.push_back(SnakePiece(rows / 2, cols / 2));

        let snake = Snake {
            gl: GlGraphics::new(opengl),
            parts: snake_parts,
            width: width,
            d: Direction::DOWN,
        };

        let food = Game::spawn_food(&snake, cols, rows);

        Game {
            gl: GlGraphics::new(opengl),
            rows: rows,
            cols: cols,
            square_width: width,
            snake: snake,
            food: food,
            just_eaten: false,
            score: 0,
            best_score: best_score,
            status: Status::NORMAL,
        }
    }

    pub fn spawn_food(snake: &Snake, cols: u32, rows: u32) -> Food {
        let mut r = rand::thread_rng();
        let new_food: Food;
        loop {
            let new_x = r.gen_range(0, cols);
            let new_y = r.gen_range(0, rows);

            if !(*snake).collides(new_x, new_y) {
                new_food = Food(new_x, new_y);
                break;
            }
        }
        new_food
    }

    pub fn render(&mut self, args: &RenderArgs) {
        const BG_COLOR: [f32; 4] = [223. / 255., 248. / 255., 235. / 255., 1.0];

        self.gl.draw(args.viewport(), |_c, gl| {
            graphics::clear(BG_COLOR, gl);
        });

        self.snake.render(args);
        self.food.render(&mut self.gl, self.square_width, args);
    }

    pub fn update(&mut self) -> bool {
        self.just_eaten = self.food.update(&mut self.snake);
        if !self.snake.update(self.just_eaten, self.rows, self.cols) {
            return false;
        }

        if self.just_eaten {
            self.score += 1;
            self.best_score = if self.score > self.best_score {
                self.score
            } else {
                self.best_score
            };
            self.food = Game::spawn_food(&self.snake, self.cols, self.rows);

            std::io::stdout()
                .write_all(format!("\rScore: {}, Best: {}", self.score, self.best_score).as_bytes())
                .expect("Writing to stdout failed");
            std::io::stdout().flush().expect("Flushing stdout failed");
        }

        true
    }

    pub fn handle_keypress(&mut self, button: piston::Button) {
        let last_direction = self.snake.d.clone();
        let is_size_1: bool = self.snake.parts.len() == 1;

        self.snake.d = match button {
            piston::Button::Keyboard(piston::Key::Up)
                if last_direction != Direction::DOWN || is_size_1 =>
            {
                Direction::UP
            }

            piston::Button::Keyboard(piston::Key::Down)
                if last_direction != Direction::UP || is_size_1 =>
            {
                Direction::DOWN
            }

            piston::Button::Keyboard(piston::Key::Right)
                if last_direction != Direction::LEFT || is_size_1 =>
            {
                Direction::RIGHT
            }

            piston::Button::Keyboard(piston::Key::Left)
                if last_direction != Direction::RIGHT || is_size_1 =>
            {
                Direction::LEFT
            }

            _ => last_direction,
        }
    }
}

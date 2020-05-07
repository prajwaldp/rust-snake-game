use opengl_graphics::GlGraphics;
use piston::input::{RenderArgs, UpdateArgs};
use rand::prelude::*;

use crate::snake::Snake;
use crate::food::Food;
use crate::helpers::Direction;

pub struct Game {
  pub gl: GlGraphics,
  pub rows: u32,
  pub cols: u32,
  pub square_width: u32,
  pub snake: Snake,
  pub food: Food,
  pub just_eaten: bool
}

impl Game {
  pub fn render(&mut self, args: &RenderArgs) {

    const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];

    self.gl.draw(args.viewport(), |c, gl| {
        graphics::clear(GREEN, gl);
    });

    self.snake.render(args);
    self.food.render(&mut self.gl, self.square_width, args);
  }

  pub fn update(&mut self, args: &UpdateArgs) {
      self.just_eaten = self.food.update(&mut self.snake);
      self.snake.update(self.just_eaten);

      if self.just_eaten {
          let mut r = rand::thread_rng();
          let new_x = r.gen_range(0, self.cols);
          let new_y = r.gen_range(0, self.rows);
          self.food = Food(new_x, new_y);
      }
  }

  pub fn handle_keypress(&mut self, button: piston::Button) {
      let last_direction = self.snake.d.clone();
      self.snake.d = match button {
          piston::Button::Keyboard(piston::Key::Up) => Direction::UP,
          piston::Button::Keyboard(piston::Key::Down) => Direction::DOWN,
          piston::Button::Keyboard(piston::Key::Right) => Direction::RIGHT,
          piston::Button::Keyboard(piston::Key::Left) => Direction::LEFT,
          _ => last_direction
      }
  }
}

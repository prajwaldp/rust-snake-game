use std::collections::LinkedList;

use opengl_graphics::GlGraphics;
use piston::input::RenderArgs;

use crate::helpers::Direction;


#[derive(Clone)]
pub struct SnakePiece(pub u32, pub u32);

pub struct Snake {
  pub gl: GlGraphics,
  pub parts: LinkedList<SnakePiece>,
  pub width: u32,
  pub d: Direction
}

impl Snake {
  pub fn render(&mut self, args: &RenderArgs) {
      const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];

      let squares: Vec<graphics::types::Rectangle> = self.parts
          .iter()
          .map(|p| SnakePiece(p.0 * self.width, p.1 * self.width))
          .map(|p| graphics::rectangle::square(p.0 as f64, p.1 as f64, self.width as f64))
          .collect();

      self.gl.draw(args.viewport(), |c, gl| {
          let transform = c.transform;

          squares
              .into_iter()
              .for_each(|square| graphics::rectangle(RED, square, transform, gl));
      })
  }

  pub fn update(&mut self, just_eaten: bool) {
      let mut new_front: SnakePiece = (*self.parts.front().expect("LinkedList is empty")).clone();
      match self.d {
          Direction::UP => new_front.1 -= 1,
          Direction::DOWN => new_front.1 += 1,
          Direction::LEFT => new_front.0 -= 1,
          Direction::RIGHT => new_front.0 += 1
      }

      if !just_eaten {
          self.parts.pop_back();
      }

      self.parts.push_front(new_front);
  }
}

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
    pub d: Direction,
}

impl Snake {
    pub fn render(&mut self, args: &RenderArgs) {
        const SNAKE_COLOR: [f32; 4] = [119. / 255., 98. / 255., 116. / 255., 1.0];

        let squares: Vec<graphics::types::Rectangle> = self
            .parts
            .iter()
            .map(|p| SnakePiece(p.0 * self.width, p.1 * self.width))
            .map(|p| graphics::rectangle::square(p.0 as f64, p.1 as f64, self.width as f64))
            .collect();

        self.gl.draw(args.viewport(), |c, gl| {
            let transform = c.transform;

            squares
                .into_iter()
                .for_each(|square| graphics::rectangle(SNAKE_COLOR, square, transform, gl));
        })
    }

    pub fn update(&mut self, just_eaten: bool, rows: u32, cols: u32) -> bool {
        let mut new_front: SnakePiece = (*self.parts.front().expect("LinkedList is empty")).clone();

        if (self.d == Direction::UP && new_front.1 == 0)
            || (self.d == Direction::LEFT && new_front.0 == 0)
            || (self.d == Direction::DOWN && new_front.1 == rows - 1)
            || (self.d == Direction::RIGHT && new_front.0 == cols - 1)
        {
            return false;
        }

        match self.d {
            Direction::UP => new_front.1 -= 1,
            Direction::DOWN => new_front.1 += 1,
            Direction::LEFT => new_front.0 -= 1,
            Direction::RIGHT => new_front.0 += 1,
        }

        if !just_eaten {
            self.parts.pop_back();
        }

        if self.collides(new_front.0, new_front.1) {
            return false;
        }

        self.parts.push_front(new_front);
        true
    }

    pub fn collides(&self, x: u32, y: u32) -> bool {
        self.parts.iter().any(|p| x == p.0 && y == p.1)
    }
}

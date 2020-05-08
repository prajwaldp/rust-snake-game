use opengl_graphics::GlGraphics;
use piston::input::RenderArgs;

use crate::snake::Snake;

pub struct Food(pub u32, pub u32);

impl Food {
    // Return true if the snake ate the food in this updates
    pub fn update(&mut self, snake: &Snake) -> bool {
        let front = snake.parts.front().unwrap();
        front.0 == self.0 && front.1 == self.1
    }

    pub fn render(&mut self, gl: &mut GlGraphics, width: u32, args: &RenderArgs) {
        const FOOD_COLOR: [f32; 4] = [1. / 255., 22. / 255., 56. / 255., 1.0];

        let x = self.0 * width;
        let y = self.1 * width;

        let square = graphics::rectangle::square(x as f64, y as f64, width as f64);

        gl.draw(args.viewport(), |c, gl| {
            let transform = c.transform;
            graphics::rectangle(FOOD_COLOR, square, transform, gl);
        });
    }
}

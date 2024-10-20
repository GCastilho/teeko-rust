use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::Canvas;
use sdl2::video::Window;

pub struct Board {
    area: Rect,
}

impl Board {
    pub fn new(area: Rect) -> Self {
        Board { area }
    }

    pub fn draw(&self, canvas: &mut Canvas<Window>) -> Result<(), String> {
        canvas.set_draw_color(Color::BLACK);

        let cell_width = self.area.w / 5;
        let cell_height = self.area.h / 5;
        for i in 0..5 {
            canvas.draw_line(
                Point::new(cell_width / 2, cell_height / 2 + i * cell_height),
                Point::new(
                    self.area.w - cell_width / 2,
                    cell_height / 2 + i * cell_height,
                ),
            )?;

            canvas.draw_line(
                Point::new(cell_width / 2 + i * cell_width, cell_height / 2),
                Point::new(
                    cell_width / 2 + i * cell_width,
                    self.area.h - cell_height / 2,
                ),
            )?;

            canvas.draw_line(
                Point::new(cell_width / 2, cell_height / 2 + i * cell_height),
                Point::new(cell_width / 2 + i * cell_width, cell_height / 2),
            )?;

            canvas.draw_line(
                Point::new(
                    cell_width / 2 + i * cell_width,
                    self.area.h - cell_height / 2,
                ),
                Point::new(
                    self.area.w - cell_width / 2,
                    cell_height / 2 + i * cell_height,
                ),
            )?;

            canvas.draw_line(
                Point::new(cell_width / 2, cell_height / 2 + i * cell_height),
                Point::new(
                    self.area.w - (cell_width / 2 + i * cell_width),
                    self.area.h - cell_height / 2,
                ),
            )?;

            canvas.draw_line(
                Point::new(cell_width / 2 + i * cell_width, cell_height / 2),
                Point::new(
                    self.area.w - cell_width / 2,
                    self.area.h - (cell_height / 2 + i * cell_height),
                ),
            )?;
        }

        Ok(())
    }
}

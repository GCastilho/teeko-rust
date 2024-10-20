use rand::distributions::Standard;
use rand::prelude::Distribution;
use rand::Rng;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::Canvas;
use sdl2::video::Window;
use std::fmt::{Display, Formatter};

#[derive(Debug, strum::Display, Clone, Copy)]
enum Piece {
    #[strum(to_string = "-")]
    None,
    #[strum(to_string = "R")]
    Red,
    #[strum(to_string = "B")]
    Black,
}

impl Distribution<Piece> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Piece {
        match rng.gen_range(0..=2) {
            0 => Piece::None,
            1 => Piece::Red,
            _ => Piece::Black,
        }
    }
}

pub struct Board {
    area: Rect,
    pieces: [[Piece; 5]; 5],
}

impl Board {
    pub fn new(area: Rect) -> Self {
        let pieces = [[Piece::None; 5]; 5];
        Board { area, pieces }
    }

    pub fn jumble(&mut self) {
        for row in &mut self.pieces {
            for piece in row {
                *piece = rand::random();
            }
        }
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

impl Display for Board {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for row in self.pieces {
            for piece in row {
                write!(f, "{}", piece)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

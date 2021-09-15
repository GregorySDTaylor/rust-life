use glutin_window::GlutinWindow;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::window::WindowSettings;
use rand::{thread_rng, Rng};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::event_loop::{EventSettings, Events};

fn main() {
    let opengl = OpenGL::V3_2;
    let mut window: GlutinWindow = WindowSettings::new("rust life", [1024, 1024])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();
    let mut game_of_life = GameOfLife {
        gl_graphics: GlGraphics::new(opengl),
        cells: [[false; 128]; 128],
    };
    game_of_life.randomize();
    let mut events = Events::new(EventSettings::new());
    while let Some(event) = events.next(&mut window) { 
        if let Some(args) = event.update_args() {
                game_of_life.update(&args);
        }
        if let Some(args) = event.render_args() {
            game_of_life.draw(&args);
        }
    }
}

struct GameOfLife {
    gl_graphics: GlGraphics,
    cells: [[bool; 128]; 128],
}

impl GameOfLife {
    fn randomize(&mut self) {
        let mut rng = thread_rng();
        for row in self.cells.iter_mut() {
            for alive in row.iter_mut() {
                *alive = rng.gen_bool(0.3);
            }
        }
    }

    fn update(&mut self, args: &UpdateArgs) {
        let previous_cells = self.cells.to_owned(); // need to learn what to_owned() really does
        for (y, row) in previous_cells.iter().enumerate() {
            for (x, alive) in row.iter().enumerate() {
                let alive_neighbors = GameOfLife::count_alive_neighbors(&previous_cells, x, y);
                self.cells[y][x] = alive_neighbors == 3 || *alive && alive_neighbors == 2;
            }
        }
    }

    fn draw(&mut self, args: &RenderArgs) {
        use graphics::*;
        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
        let square = rectangle::square(0.0, 0.0, 8.0);
        self.gl_graphics.draw(args.viewport(), |context, graphics| {
            clear(BLACK, graphics);
        });
        for (x, row) in self.cells.iter().enumerate() {
            for (y, alive) in row.iter().enumerate() {
                if *alive {
                    self.gl_graphics.draw(args.viewport(), |context, graphics| {
                    let transform = context
                        .transform
                        .trans((x * 8) as f64, (y * 8) as f64);
                    rectangle(GREEN, square, transform, graphics);
                    });
                }
            }
        }
    }

    fn count_alive_neighbors(cells: &[[bool; 128]; 128], x: usize, y: usize) -> usize {
        // how do I handle this array argument with variable length?
        let above = if y == 0 { cells.len() - 1 } else { y - 1 };
        let below = if y == cells.len() - 1 { 0 } else { y + 1 };
        let left = if x == 0 { cells[0].len() - 1 } else { x - 1 };
        let right = if x == cells[0].len() - 1 { 0 } else { x + 1 };
        let neighbors = [
            cells[above][left],
            cells[above][x],
            cells[above][right],
            cells[y][left],
            cells[y][right],
            cells[below][left],
            cells[below][x],
            cells[below][right],
        ];
        return neighbors.iter().filter(|alive| **alive).count(); // learn what is a double borrow?
    }
}

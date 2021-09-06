use rand::{thread_rng, Rng};
use tokio::time::{sleep_until, Duration, Instant};

#[tokio::main]
async fn main() {
    let mut next_tick = Instant::now();
    let mut game_of_life = GameOfLife {
        cells: [[false; 64]; 32],
    };
    game_of_life.randomize();
    loop {
        next_tick += Duration::from_millis(200);
        game_of_life.update();
        game_of_life.draw();
        if next_tick > Instant::now() {
            sleep_until(next_tick).await;
        } else {
            next_tick = Instant::now();
        }
    }
}

struct GameOfLife {
    cells: [[bool; 64]; 32],
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

    fn update(&mut self) {
        let previous_cells = self.cells.to_owned(); // need to learn what to_owned() really does
        for (y, row) in previous_cells.iter().enumerate() {
            for (x, alive) in row.iter().enumerate() {
                let alive_neighbors = GameOfLife::count_alive_neighbors(&previous_cells, x, y);
                self.cells[y][x] = alive_neighbors == 3 || *alive && alive_neighbors == 2;
            }
        }
    }

    fn draw(&self) {
        let mut string_cells = String::new();
        string_cells.push_str("\x1B[2J\x1B[1;1H"); // clears screen
        for row in self.cells.iter() {
            for alive in row.iter() {
                string_cells.push_str(if *alive { "O" } else { " " });
            }
            string_cells.push('\n');
        }
        println!("{}", string_cells);
    }

    fn count_alive_neighbors(cells: &[[bool; 64]; 32], x: usize, y: usize) -> usize {
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

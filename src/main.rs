use rand::{thread_rng, Rng};
use tokio::time::{sleep_until, Duration, Instant};

#[tokio::main]
async fn main() {
    let mut next_tick = Instant::now();
    let mut game_of_life = GameOfLife {
        cells: [[false; 8]; 8],
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
    cells: [[bool; 8]; 8],
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
        let previous_cells = self.cells;
    }

    fn draw(&self) {
        let mut string_cells = String::new();
        string_cells.push_str("\x1B[2J\x1B[1;1H"); // clears screen
        for row in self.cells.iter() {
            for alive in row.iter() {
                string_cells.push_str(if *alive { " ◼" } else { " ◻" });
            }
            string_cells.push_str("\n"); // new line
        }
        println!("{}", string_cells);
    }
}



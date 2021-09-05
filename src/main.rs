use tokio::time::{sleep_until, Instant, Duration};

#[tokio::main]
async fn main() {
    let mut next_tick = Instant::now();
    loop {
        update(&mut next_tick);
        draw();
        sleep_until(next_tick).await;
    }
}

fn update(next_tick: &mut Instant) {
    *next_tick += Duration::from_millis(200);
}

fn draw() {
    print!("\x1B[2J\x1B[1;1H");
    println!("◻ ◼ ◻ ◼ ◻
◼ ◻ ◼ ◻ ◼
◻ ◼ ◻ ◼ ◻
◼ ◻ ◼ ◻ ◼
◻ ◼ ◻ ◼ ◻");
}

# Game of Life

Example usage:
```rust,no_run
extern crate game_of_life;

use game_of_life::*;

use std::{thread, time};

fn main() {
    let mut current_field = GameOfLife::new(10);

    for x in 0 .. 10 {
        if x == 5 {
            continue;
        }
        let position = Position{x:x,y:5};
        current_field.set_state(position,State::Alive);
    }

    for iter in 0..100 {
        println!("Game Field Iteration {}\n{}", iter, current_field);
        let new_field = current_field.get_next_generation();

        if new_field == current_field {
            println!("No more changes");
            break;
        }
        current_field = new_field;

        thread::sleep(time::Duration::from_millis(300));
    }
}
```
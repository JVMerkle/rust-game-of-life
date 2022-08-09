# Game of Life

Simple library.

## Example

```rust
extern crate game_of_life;

use game_of_life::*;

use std::{thread, time};

fn main() {
    let mut field = GameOfLife::new(10);

    // Initialize the field
    for x in 0 .. 10 {
        if x == 5 {
            continue;
        }
        let position = Position{x:x,y:5};
        field.set_state(position,State::Alive);
    }

    // Run!
    for iter in 0..10 {
        println!("Game Field Iteration {}\n{}", iter, field);
        let new_field = field.get_next_generation();

        if new_field == field {
            println!("No more changes");
            break;
        }
        field = new_field;

        thread::sleep(time::Duration::from_millis(300));
    }
}
```

mod game_rule;

use std::ops::Add;
use std::fmt;
use std::collections::HashMap;

#[derive(Copy, Clone, Hash, Eq, PartialEq, Debug)]
pub struct Position{
    pub x: i32,
    pub y: i32
}

impl Add for Position {
    type Output = Position;

    fn add(self, other: Position) -> Position {
        Position { x: self.x + other.x, y: self.y + other.y }
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum State {
    Alive,
    Dead
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let printable = match *self {
            State::Alive => 'x',
            State::Dead => ' ',
        };
        write!(f, "{}", printable)
    }
}

// Gamefield

#[derive(PartialEq)]
pub struct GameOfLife {
    hash_map: HashMap<Position, State>,
    length : i32
}

impl fmt::Display for GameOfLife {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut field = String::new();

        for x in 0..self.len() {
            for y in 0..self.len() {
                let position = Position { y: y, x: x };
                let current_state = match self.get_state(&position) {
                    None => State::Dead,
                    Some(value) => *value,
                };
                field.push_str(&current_state.to_string());
            }
            field.push_str("\n");
        }
        write!(f, "{}", field)
    }
}

impl GameOfLife {
    pub fn new(length : i32) -> GameOfLife {
        if length < 1 {
            panic!("GOL field length must be larger than zero");
        }
        let mut thing = GameOfLife { hash_map: HashMap::new() , length: length};
        for x in 0..length{
            for y in 0..length {
                let position = Position { y: y, x: x };
                thing.hash_map.insert(position, State::Dead);
            }
        }
        return thing;
    }

    pub fn get_state(&self, pos:&Position) -> Option<&State> {
        return self.hash_map.get(pos);
    }

    pub fn set_state(&mut self, pos:Position, state:State) {
        *self.hash_map.entry(pos).or_insert(State::Dead) = state;
    }

    pub fn get_live_neighbours(&self, position:Position) -> u32 {
        let mut alive = 0;
        for x in -1..2 {
            for y in -1..2 {
                let mask = Position { y: y, x: x };
                let neighbour = position + mask;

                // Ignore own position
                if neighbour == position {
                    continue;
                }

                let state = match self.get_state(&neighbour) {
                    None => State::Dead,
                    Some(value) => *value,
                };
                if state == State::Alive {
                    alive += 1;
                }
            }
        }
        return alive;
    }

    pub fn len(&self) -> i32 {
        self.length
    }

    pub fn get_next_generation(&mut self) -> GameOfLife {
        let mut new_field = GameOfLife::new(self.len());

        for (position, current_state) in self.hash_map.iter() {
            let neighbours = self.get_live_neighbours(*position);
            let new_state = game_rule::get_next_state(*current_state, neighbours);
            new_field.set_state(*position,new_state);
            }

        return new_field;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init_all_dead() {
        let field = GameOfLife::new(50);
        let mut i = 0;
        for (.. , state) in &field.hash_map {
            assert_eq!(*state, State::Dead);
            i += 1;
        }
        assert_eq!(i, 50 * 50);
    }

    #[test]
    fn live_neighbours() {
        let mut field = GameOfLife::new(10);
        field.set_state(Position{x:0, y:0}, State::Alive);
        field.set_state(Position{x:1, y:1}, State::Alive);
        field.set_state(Position{x:2, y:2}, State::Alive);
        assert_eq!(field.get_live_neighbours(Position{x:0, y:0}), 1);
        assert_eq!(field.get_live_neighbours(Position{x:1, y:1}), 2);
    }

    #[test]
    fn field_size() {
        let field = GameOfLife::new(5);
        assert_eq!(field.hash_map.len(), 25);
        assert_eq!(field.len(), 5);
    }

    #[test]
    #[should_panic]
    fn empty_field() {
        let _field = GameOfLife::new(0);
    }
}


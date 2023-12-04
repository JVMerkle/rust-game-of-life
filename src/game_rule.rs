use super::State;

pub fn get_next_state(current: State, neighbours: u32) -> State {
    if neighbours <= 1 || neighbours >= 4 {
        return State::Dead;
    } else if neighbours == 3 {
        return State::Alive;
    }

    return current;
}

#[cfg(test)]
mod tests {
    use super::*;

    /*
    1. Any live cell with fewer than two live neighbours dies, as if caused by underpopulation.
    2. Any live cell with two or three live neighbours lives on to the next generation.
    3. Any live cell with more than three live neighbours dies, as if by overpopulation.
    4. Any dead cell with exactly three live neighbours becomes a live cell, as if by reproduction.
    */

    #[test]
    fn underpopulation() {
        assert_eq!(get_next_state(State::Alive, 0), State::Dead);
        assert_eq!(get_next_state(State::Alive, 1), State::Dead);
    }

    #[test]
    fn survive() {
        assert_eq!(get_next_state(State::Alive, 2), State::Alive);
        assert_eq!(get_next_state(State::Alive, 3), State::Alive);
    }

    #[test]
    fn overpopulation() {
        assert_eq!(get_next_state(State::Alive, 4), State::Dead);
        assert_eq!(get_next_state(State::Alive, 5), State::Dead);
        assert_eq!(get_next_state(State::Alive, 6), State::Dead);
        assert_eq!(get_next_state(State::Alive, 7), State::Dead);
        assert_eq!(get_next_state(State::Alive, 8), State::Dead);
    }

    #[test]
    fn reproduction() {
        assert_eq!(get_next_state(State::Dead, 3), State::Alive);
    }
}

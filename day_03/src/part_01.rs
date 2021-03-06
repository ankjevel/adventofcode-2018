use std::io::Result;

use crate::{Input, Square::Tree};

pub fn main(input: &Input) -> Result<usize> {
    let max = input.get(0).unwrap().len();
    let mut index = 0;
    Ok(input
        .into_iter()
        .map(|row| {
            let square = row.get(index).unwrap();

            index = (index + 3) % max;

            square.to_owned()
        })
        .filter(|square| square == &Tree)
        .count())
}

#[cfg(test)]
mod tests {
    use crate::parse_input;

    use super::*;

    const EXAMPLE_DATA: &'static str = "
        ..##.......
        #...#...#..
        .#....#..#.
        ..#.#...#.#
        .#...##..#.
        ..#.##.....
        .#.#.#....#
        .#........#
        #.##...#...
        #...##....#
        .#..#...#.#
    ";

    #[test]
    fn it_gets_the_example_correct() -> Result<()> {
        assert_eq!(main(&parse_input(EXAMPLE_DATA))?, 7);
        Ok(())
    }
}

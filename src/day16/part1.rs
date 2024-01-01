use std::fs::File;
use std::io::Read;

use std::collections::HashSet;


pub fn main(filename: &str) -> String {
    let mut file = File::open(filename).expect("Failed to open file");
	let mut file_content: String = String::new();

	file.read_to_string(&mut file_content).expect("Failed to read file content");

    let floor: Vec<Vec<char>> = file_content
		.lines()
		.map(|line| line
			.chars()
			.collect())
		.collect();

    get_sum_of_energized_tiles(&floor).to_string()
}


#[derive(Debug, Hash, PartialEq, Eq, Clone)]
enum Direction {
    Up,
    Right,
	Down,
    Left
}


impl Direction {
    fn get_coords_addition(&self) -> (i8, i8) {
        match self {
            Direction::Up => (-1, 0),
            Direction::Right => (0, 1),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1)
        }
    }
}


fn get_sum_of_energized_tiles(sequences: &Vec<Vec<char>>) -> usize {
    let mut queue: Vec<((isize, isize), Direction)> = Vec::new();
    let mut visited_tiles: HashSet<((isize, isize), Direction)> = HashSet::new();
    let mut energized_tiles: HashSet<(isize, isize)> = HashSet::new();

    queue.push(((65, 43), Direction::Down));

    let max_x: isize = sequences[0].len() as isize;
    let max_y: isize = sequences.len() as isize;

    while !queue.is_empty() {
        let item: ((isize, isize), Direction) = queue.pop().unwrap();

        if item.0.0 < 0 || item.0.1 < 0 || max_x <= item.0.0 || max_y <= item.0.1 {
            continue;
        }

        if visited_tiles.contains(&item) {
            continue
        }

        visited_tiles.insert(item.clone());
        energized_tiles.insert(item.0.clone());

        match sequences[item.0.0 as usize][item.0.1 as usize] {
            '-' => {
                if item.1 == Direction::Right || item.1 == Direction::Left {
                    let coords_addition: (i8, i8) = item.1.get_coords_addition();
                    queue.push(((item.0.0 + coords_addition.0 as isize, item.0.1 + coords_addition.1 as isize), item.1));
                    continue;
                }

                let coords_addition: (i8, i8) = Direction::Right.get_coords_addition();
                queue.push(((item.0.0 + coords_addition.0 as isize, item.0.1 + coords_addition.1 as isize), Direction::Right));

                let coords_addition: (i8, i8) = Direction::Left.get_coords_addition();
                queue.push(((item.0.0 + coords_addition.0 as isize, item.0.1 + coords_addition.1 as isize), Direction::Left));
            },
            '|' => {
                if item.1 == Direction::Up || item.1 == Direction::Down {
                    let coords_addition: (i8, i8) = item.1.get_coords_addition();
                    queue.push(((item.0.0 + coords_addition.0 as isize, item.0.1 + coords_addition.1 as isize), item.1));
                    continue;
                }

                let coords_addition: (i8, i8) = Direction::Up.get_coords_addition();
                queue.push(((item.0.0 + coords_addition.0 as isize, item.0.1 + coords_addition.1 as isize), Direction::Up));

                let coords_addition: (i8, i8) = Direction::Down.get_coords_addition();
                queue.push(((item.0.0 + coords_addition.0 as isize, item.0.1 + coords_addition.1 as isize), Direction::Down));
            }
            '/' => {
                match item.1 {
                    Direction::Up => {
                        let coords_addition: (i8, i8) = Direction::Right.get_coords_addition();
                        queue.push(((item.0.0 + coords_addition.0 as isize, item.0.1 + coords_addition.1 as isize), Direction::Right));
                    },
                    Direction::Right => {
                        let coords_addition: (i8, i8) = Direction::Up.get_coords_addition();
                        queue.push(((item.0.0 + coords_addition.0 as isize, item.0.1 + coords_addition.1 as isize), Direction::Up));
                    },
                    Direction::Down => {
                        let coords_addition: (i8, i8) = Direction::Left.get_coords_addition();
                        queue.push(((item.0.0 + coords_addition.0 as isize, item.0.1 + coords_addition.1 as isize), Direction::Left));
                    },
                    Direction::Left => {
                        let coords_addition: (i8, i8) = Direction::Down.get_coords_addition();
                        queue.push(((item.0.0 + coords_addition.0 as isize, item.0.1 + coords_addition.1 as isize), Direction::Down));
                    },
                }
            }
            '\\' => {
                match item.1 {
                    Direction::Up => {
                        let coords_addition: (i8, i8) = Direction::Left.get_coords_addition();
                        queue.push(((item.0.0 + coords_addition.0 as isize, item.0.1 + coords_addition.1 as isize), Direction::Left));
                    },
                    Direction::Right => {
                        let coords_addition: (i8, i8) = Direction::Down.get_coords_addition();
                        queue.push(((item.0.0 + coords_addition.0 as isize, item.0.1 + coords_addition.1 as isize), Direction::Down));
                    },
                    Direction::Down => {
                        let coords_addition: (i8, i8) = Direction::Right.get_coords_addition();
                        queue.push(((item.0.0 + coords_addition.0 as isize, item.0.1 + coords_addition.1 as isize), Direction::Right));
                    },
                    Direction::Left => {
                        let coords_addition: (i8, i8) = Direction::Up.get_coords_addition();
                        queue.push(((item.0.0 + coords_addition.0 as isize, item.0.1 + coords_addition.1 as isize), Direction::Up));
                    },
                }
            }
            _ => {
                let coords_addition: (i8, i8) = item.1.get_coords_addition();
                queue.push(((item.0.0 + coords_addition.0 as isize, item.0.1 + coords_addition.1 as isize), item.1))
            }
        }
    }

    energized_tiles.len()
}
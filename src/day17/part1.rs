use std::fs::File;
use std::io::Read;

use std::collections::HashMap;


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

    fn get_variants() -> Vec<Direction> {
        vec![Direction::Up, Direction::Left, Direction::Right, Direction::Down]
    }
}


pub fn main(filename: &str) -> String {
    let mut file = File::open(filename).expect("Failed to open file");
	let mut file_content: String = String::new();

	file.read_to_string(&mut file_content).expect("Failed to read file content");

    let heatmap: Vec<Vec<usize>> = file_content
		.lines()
		.map(|line| line
			.chars()
            .map(|c| c.to_digit(10).unwrap() as usize)
			.collect())
		.collect();

    get_least_heat_loss(&heatmap).to_string()
}


fn get_least_heat_loss(heatmap: &Vec<Vec<usize>>) -> usize {
    let mut visited_blocks: HashMap<((usize, usize), Direction, u8), usize> = HashMap::new();
    let mut queue: Vec<(((usize, usize), Direction, u8), usize)> = Vec::new();

    let max_coords: (usize, usize) = (heatmap.len() - 1, heatmap[0].len() - 1);
    let mut min_heat_loss: usize = usize::MAX;

    queue.push((((1, 0), Direction::Down, 1), heatmap[1][0]));
    queue.push((((0, 1), Direction::Right, 1), heatmap[0][1]));

    while !queue.is_empty() {
        let item: (((usize, usize), Direction, u8), usize) = queue.pop().unwrap();

        if visited_blocks.contains_key(&item.0) {
            if *visited_blocks.get(&item.0).unwrap() <= item.1 {
                continue;
            }
        }

        if item.1 >= min_heat_loss {
            continue;
        }

        if item.0.0 == (max_coords.0, max_coords.1) {
            min_heat_loss = item.1;
            continue;
        }

        visited_blocks.insert(item.0.clone(), item.1);

        if item.0.2 < 3 {
            let coords_addition = item.0.1.get_coords_addition();
            if is_valid_mode(item.0.0, max_coords, coords_addition) {
                let new_coord: (usize, usize) = (
                    (item.0.0.0 as isize + coords_addition.0 as isize) as usize, 
                    (item.0.0.1 as isize + coords_addition.1 as isize) as usize
                );
                queue.push((((new_coord.0, new_coord.1), item.0.1.clone(), item.0.2 + 1), item.1 + heatmap[new_coord.0][new_coord.1]));
            }
        }

        for new_direction in Direction::get_variants() {
            if new_direction == item.0.1 {
                continue;
            }

            let coords_addition = new_direction.get_coords_addition();

            if item.0.1.get_coords_addition() == (-coords_addition.0, -coords_addition.1) {
                continue;
            }
            
            if is_valid_mode(item.0.0, max_coords, coords_addition) {
                let new_coord: (usize, usize) = (
                    (item.0.0.0 as isize + coords_addition.0 as isize) as usize, 
                    (item.0.0.1 as isize + coords_addition.1 as isize) as usize
                );
                queue.push((((new_coord.0, new_coord.1), new_direction, 1), item.1 + heatmap[new_coord.0][new_coord.1]));
            }
        }
    }

    min_heat_loss
}


fn is_valid_mode(coords: (usize, usize), max_coords: (usize, usize), coords_addition: (i8, i8)) -> bool {
    if coords.0 == 0 && coords_addition.0 == -1 {
        return false;
    }

    if coords.1 == 0 && coords_addition.1 == -1 {
        return false;
    }

    if coords.0 == max_coords.0 && coords_addition.0 == 1 {
        return false;
    }

    if coords.1 == max_coords.1 && coords_addition.1 == 1 {
        return false;
    }

    true
}
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
    fn get_coords_addition(&self) -> (isize, isize) {
        match self {
            Direction::Up => (-1, 0),
            Direction::Right => (0, 1),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1)  
        }
    }
}


pub fn main(filename: &str) -> String {
    let mut file = File::open(filename).expect("Failed to open file");
	let mut file_content: String = String::new();

	file.read_to_string(&mut file_content).expect("Failed to read file content");

    let dig_plan: Vec<(Direction, isize)> = file_content
		.lines()
		.map(|line| {
                let mut hexadecimal_codes: Vec<char> = line
                    .split_whitespace()
                    .nth(2)
                    .map(|s| 
                        s
                            .chars()
                            .skip(2)
                            .take(
                                s
                                    .len()
                                    .saturating_sub(3)
                            )
                            .collect()
                    )
                    .unwrap();
                
                let direction: Direction = get_direction(hexadecimal_codes.pop().unwrap().to_digit(10).unwrap());
                let hexadecimal_number = hexadecimal_codes.into_iter().collect::<String>();
                (
                    direction,
                    isize::from_str_radix(hexadecimal_number.as_str(), 16).unwrap()
                )
            }
        )
		.collect();

    get_area(&dig_plan).to_string()
}


fn get_direction(id: u32) -> Direction {
    match id {
        0 => Direction::Right,
        1 => Direction::Down,
        2 => Direction::Left,
        _ => Direction::Up
    }
}


fn get_area(dig_plan: &Vec<(Direction, isize)>) -> usize {
    let points: Vec<(isize, isize)> = get_points(&dig_plan);
    let perimeter: usize = dig_plan
        .iter()
        .map(|(_, y)| y)
        .sum::<isize>() as usize;

    get_shoelace_area(&points) - (perimeter / 2) + 1 + perimeter 
}


fn get_points(dig_plan: &Vec<(Direction, isize)>) -> Vec<(isize, isize)> {
    let mut points: Vec<(isize, isize)> = Vec::new();
    let mut position: (isize, isize) = (0, 0);
    points.push((0, 0));

    let coords_addition: HashMap<Direction, (isize, isize)> = HashMap::from(
        [
            (Direction::Up, Direction::Up.get_coords_addition()),
            (Direction::Right, Direction::Right.get_coords_addition()),
            (Direction::Down, Direction::Down.get_coords_addition()),
            (Direction::Left, Direction::Left.get_coords_addition())
        ]
    );

    for instruction in dig_plan {
        position.0 += coords_addition.get(&instruction.0).unwrap().0 * instruction.1; 
        position.1 += coords_addition.get(&instruction.0).unwrap().1 * instruction.1;
        points.push(position);
    }

    points
}

fn get_shoelace_area(vertices: &Vec<(isize, isize)>) -> usize {
    let n = vertices.len();
    
    (0..n).map(|i| {
        let x1 = vertices[i].0;
        let y1 = vertices[i].1;
        let x2 = vertices[(i + 1) % n].0;
        let y2 = vertices[(i + 1) % n].1;
        x1 * y2 - x2 * y1
    }).sum::<isize>().abs() as usize / 2
}
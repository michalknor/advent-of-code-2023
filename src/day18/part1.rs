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
    let mut file: File = File::open(filename).expect("Failed to open file");
	let mut file_content: String = String::new();

	file.read_to_string(&mut file_content).expect("Failed to read file content");

    let dig_plan: Vec<(char, isize)> = file_content
		.lines()
		.map(|line| {
                let splitted_line: Vec<&str> = line.split(" ").collect();
                (splitted_line[0].chars().collect::<Vec<char>>()[0], splitted_line[1].parse::<isize>().unwrap())
            }
        )
		.collect();

    get_area(&dig_plan).to_string()
}


fn get_area(dig_plan: &Vec<(char, isize)>) -> usize {
    let boundary_coords: ((isize, isize), (isize, isize)) = get_boundary_coords(&get_points(&dig_plan));

    let mut dig_area: Vec<Vec<bool>> = get_dig_area(&dig_plan, &boundary_coords);

    let perimeter = dig_area
        .iter()
        .flatten()
        .filter(|&&x| x)
        .count();

    let mut stack: Vec<(usize, usize)> = Vec::new();

    for i in 0..dig_area.len() {
        stack.push((i, 0));
        stack.push((i, dig_area[1].len()-1));
    }
    for i in 1..dig_area[1].len()-1 {
        stack.push((0, i));
        stack.push((dig_area.len()-1, i));
    }

    while !stack.is_empty() {
        let coord: (usize, usize) = stack.pop().unwrap();

        if dig_area[coord.0][coord.1] {
            continue;
        }

        dig_area[coord.0][coord.1] = true;

        if coord.0 > 0 {
            stack.push((coord.0 - 1, coord.1));
        }

        if coord.0 < dig_area.len()-1 {
            stack.push((coord.0 + 1, coord.1));
        }

        if coord.1 > 0 {
            stack.push((coord.0, coord.1 - 1));
        }

        if coord.1 < dig_area[0].len()-1 {
            stack.push((coord.0, coord.1 + 1));
        }
    }

    dig_area.len() * dig_area[0].len() - dig_area
        .iter()
        .flatten()
        .filter(|&&x| x)
        .count() + perimeter
}


fn get_points(dig_plan: &Vec<(char, isize)>) -> Vec<(isize, isize)> {
    let mut points: Vec<(isize, isize)> = Vec::new();
    let mut position: (isize, isize) = (0, 0);
    points.push((0, 0));

    let coords_addition: HashMap<char, (isize, isize)> = HashMap::from(
        [
            ('U', Direction::Up.get_coords_addition()),
            ('R', Direction::Right.get_coords_addition()),
            ('D', Direction::Down.get_coords_addition()),
            ('L', Direction::Left.get_coords_addition())
        ]
    );

    for instruction in dig_plan {
        position.0 += coords_addition.get(&instruction.0).unwrap().0 * instruction.1; 
        position.1 += coords_addition.get(&instruction.0).unwrap().1 * instruction.1;
        points.push(position);
    }

    points
}


fn get_boundary_coords(points: &Vec<(isize, isize)>) -> ((isize, isize), (isize, isize)) {
    points
        .iter()
        .skip(1)
        .fold((points[0], points[0]), |boundary_coords, &x| 
            if x.0 < boundary_coords.0.0 {
                ((x.0, boundary_coords.0.1), boundary_coords.1)
            }
            else if x.1 < boundary_coords.0.1 {
                ((boundary_coords.0.0, x.1), boundary_coords.1)
            }
            else if x.0 > boundary_coords.1.0 {
                (boundary_coords.0, (x.0, boundary_coords.1.1))
            }
            else if x.1 > boundary_coords.1.1 {
                (boundary_coords.0, (boundary_coords.1.0, x.1))
            } 
            else {
                boundary_coords
            }
        )
}


fn get_dig_area(dig_plan: &Vec<(char, isize)>, boundary_coords: &((isize, isize), (isize, isize))) -> Vec<Vec<bool>> {
    let mut dig_area: Vec<Vec<bool>> = vec![
        vec![
            false; (boundary_coords.1.1 - boundary_coords.0.1 + 1) as usize
        ]; 
        (boundary_coords.1.0 - boundary_coords.0.0 + 1) as usize
    ];

    let coords_addition: HashMap<char, (isize, isize)> = HashMap::from(
        [
            ('U', Direction::Up.get_coords_addition()),
            ('R', Direction::Right.get_coords_addition()),
            ('D', Direction::Down.get_coords_addition()),
            ('L', Direction::Left.get_coords_addition())
        ]
    );

    let mut current_position: (usize, usize) = (-boundary_coords.0.0 as usize, -boundary_coords.0.1 as usize);

    for instruction in dig_plan {
        for _ in 0..instruction.1 {
            current_position.0 = (current_position.0 as isize + coords_addition.get(&instruction.0).unwrap().0) as usize; 
            current_position.1 = (current_position.1 as isize + coords_addition.get(&instruction.0).unwrap().1) as usize;
            dig_area[current_position.0][current_position.1] = true;
        }
    }

    dig_area
}
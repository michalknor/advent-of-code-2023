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
    let points: Vec<(isize, isize)> = get_points(&dig_plan);

    let boundary_coords: ((isize, isize), (isize, isize)) = points
        .iter()
        .skip(1)
        .fold((points[0], points[0]), |boundary_coords, &x| 
            if x.0 < boundary_coords.0.0 {
                return ((x.0, boundary_coords.0.1), boundary_coords.1)
            }
            else if x.1 < boundary_coords.0.1 {
                return ((boundary_coords.0.0, x.1), boundary_coords.1)
            }
            else if x.0 > boundary_coords.1.0 {
                return (boundary_coords.0, (x.0, boundary_coords.1.1))
            }
            else if x.0 > boundary_coords.1.1 {
                return (boundary_coords.0, (boundary_coords.0.0, x.1))
            } 
            else {
                boundary_coords
            }
        );

    
    //maximum - minimum + 1

    0
}


fn get_points(dig_plan: &Vec<(char, isize)>) -> Vec<(isize, isize)> {
    let mut points: Vec<(isize, isize)> = Vec::new();
    let mut position: (isize, isize) = (0, 0);
    points.push((0, 0));

    let coords_addition: HashMap<char, (isize, isize)> = HashMap::from([
        ('U', Direction::Up.get_coords_addition()),
        ('R', Direction::Right.get_coords_addition()),
        ('D', Direction::Down.get_coords_addition()),
        ('L', Direction::Left.get_coords_addition())
    ]);

    for instruction in dig_plan {
        position.0 += coords_addition.get(&instruction.0).unwrap().0 * instruction.1; 
        position.1 += coords_addition.get(&instruction.0).unwrap().1 * instruction.1;
        points.push(position);
    }

    points
}
use std::fs::File;
use std::io::Read;


pub fn main(filename: &str) -> String {
    let mut file = File::open(filename).expect("Failed to open file");
	let mut file_content: String = String::new();

	file.read_to_string(&mut file_content).expect("Failed to read file content");

    let map: Vec<Vec<char>> = file_content
        .lines()
        .map(|line| 
            line.chars().collect()
        )
        .collect();

    get_longest_path(&map).to_string()
}


fn get_longest_path(map: &Vec<Vec<char>>) -> usize {
    let finish_coords: (usize, usize) = (map.len()-1, map[0].len()-2);

    let mut stack: Vec<((usize, usize), Vec<(usize, usize)>)> = vec![((1, 1), vec![(0, 1)])];
    let mut max_steps: usize = usize::MIN;

    let (finish_coords, remaining_steps) = get_crossroad_coord(&map, (finish_coords.0-1, finish_coords.1), vec![finish_coords]);

    while !stack.is_empty() {
        let (current_coord, mut history) = stack.pop().unwrap();

        if history.contains(&current_coord) {
            continue;
        }

        if current_coord == finish_coords {
            if history.len() > max_steps {
                max_steps = history.len()
            }
            println!("{}, {}", max_steps, stack.len());
            continue;
        }
        
        history.push(current_coord);

        if map[current_coord.0][current_coord.1+1] != '#' {
            stack.push(((current_coord.0, current_coord.1+1), history.clone()))
        }
        if map[current_coord.0+1][current_coord.1] != '#' {
            stack.push(((current_coord.0+1, current_coord.1), history.clone()))
        }
        if map[current_coord.0][current_coord.1-1] != '#' {
            stack.push(((current_coord.0, current_coord.1-1), history.clone()))
        }
        if map[current_coord.0-1][current_coord.1] != '#' {
            stack.push(((current_coord.0-1, current_coord.1), history.clone()))
        }
    }

    max_steps + remaining_steps
}


fn get_crossroad_coord(map: &Vec<Vec<char>>, coord: (usize, usize), history: Vec<(usize, usize)>) -> ((usize, usize), usize) {
    let mut stack: Vec<((usize, usize), Vec<(usize, usize)>)> = vec![(coord, history)];

    let mut current_coord: (usize, usize) = (0, 0);
    let mut history: Vec<(usize, usize)> = Vec::new();

    while stack.len() <= 1 {
        (current_coord, history) = stack.pop().unwrap();

        if history.contains(&current_coord) {
            continue;
        }
        
        history.push(current_coord);

        if map[current_coord.0][current_coord.1+1] != '#' && !history.contains(&(current_coord.0, current_coord.1+1)) {
            stack.push(((current_coord.0, current_coord.1+1), history.clone()))
        }
        if map[current_coord.0+1][current_coord.1] != '#' && !history.contains(&(current_coord.0+1, current_coord.1)) {
            stack.push(((current_coord.0+1, current_coord.1), history.clone()))
        }
        if map[current_coord.0][current_coord.1-1] != '#' && !history.contains(&(current_coord.0, current_coord.1-1)) {
            stack.push(((current_coord.0, current_coord.1-1), history.clone()))
        }
        if map[current_coord.0-1][current_coord.1] != '#' && !history.contains(&(current_coord.0-1, current_coord.1)) {
            stack.push(((current_coord.0-1, current_coord.1), history.clone()))
        }
    }
    
    ((current_coord), history.len() - 1)
}
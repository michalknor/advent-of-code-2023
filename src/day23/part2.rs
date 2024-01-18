use std::fs::File;
use std::io::Read;

use std::collections::HashMap;
use std::collections::HashSet;


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
    let mut max_steps: usize = usize::MIN;

    let mut nodes: HashMap<(usize, usize), HashSet<((usize, usize), usize)>> = HashMap::new();

    let (starting_coords, starting_remaining_steps) = get_crossroad_coord(&map, (1, 1), vec![(0, 1)]);
    let (finish_coords, finish_remaining_steps) = get_crossroad_coord(&map, (finish_coords.0-1, finish_coords.1), vec![finish_coords]);

    let mut stack: Vec<((usize, usize), (usize, usize), (usize, usize), usize)> = vec![(finish_coords, (0, 1), finish_coords, 0)];

    while !stack.is_empty() {
        let (current_coord, last_coord, mut last_node, mut length) = stack.pop().unwrap();

        if current_coord == (map.len()-1, map[0].len()-2) || current_coord == (1, 1) {
            continue;
        }

        length += 1;

        let mut n_of_paths: u8 = 0;

        if map[current_coord.0][current_coord.1+1] != '#' {
            n_of_paths += 1;
        }
        if map[current_coord.0+1][current_coord.1] != '#' {
            n_of_paths += 1;
        }
        if map[current_coord.0][current_coord.1-1] != '#' {
            n_of_paths += 1;
        }
        if map[current_coord.0-1][current_coord.1] != '#' {
            n_of_paths += 1;
        }

        if n_of_paths > 2 {
            if nodes.keys().any(|key| *key == current_coord) {
                nodes.entry(current_coord).or_insert(HashSet::from([(last_node, length)])).insert((last_node, length));
                nodes.entry(last_node).or_insert(HashSet::from([(current_coord, length)])).insert((current_coord, length));

                continue;
            }
            nodes.entry(current_coord).or_insert(HashSet::from([(last_node, length)])).insert((last_node, length));
            nodes.entry(last_node).or_insert(HashSet::from([(current_coord, length)])).insert((current_coord, length));

            last_node = current_coord;

            length = 0;
        }
        
        if map[current_coord.0][current_coord.1+1] != '#' && (current_coord.0, current_coord.1+1) != last_coord {
            stack.push(((current_coord.0, current_coord.1+1), current_coord, last_node, length));
        }
        if map[current_coord.0+1][current_coord.1] != '#' && (current_coord.0+1, current_coord.1) != last_coord {
            stack.push(((current_coord.0+1, current_coord.1), current_coord, last_node, length));
        }
        if map[current_coord.0][current_coord.1-1] != '#' && (current_coord.0, current_coord.1-1) != last_coord {
            stack.push(((current_coord.0, current_coord.1-1), current_coord, last_node, length));
        }
        if map[current_coord.0-1][current_coord.1] != '#' && (current_coord.0-1, current_coord.1) != last_coord {
            stack.push(((current_coord.0-1, current_coord.1), current_coord, last_node, length));
        }
    }

    let mut stack: Vec<((usize, usize), HashSet<(usize, usize)>, usize)> = vec![((starting_coords), HashSet::new(), 0)];

    while !stack.is_empty() {
        let (current_node, mut visited_nodes, length) = stack.pop().unwrap();

        if current_node == finish_coords {
            if length > max_steps {
                max_steps = length;
                println!("{}", length+ starting_remaining_steps + finish_remaining_steps);
            }
        }

        visited_nodes.insert(current_node);

        for node in nodes.get(&current_node).unwrap() {
            if !visited_nodes.contains(&node.0) {
                stack.push((node.0, visited_nodes.clone(), length + node.1));
            }
        }
    }

    max_steps + starting_remaining_steps + finish_remaining_steps
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
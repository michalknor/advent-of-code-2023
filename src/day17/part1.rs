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

    println!("{:?}", heatmap);

    get_least_heat_loss(&heatmap).to_string()
}


fn get_least_heat_loss(heatmap: &Vec<Vec<usize>>) -> usize {
    let mut visited_blocks: HashMap<(usize, usize), usize> = HashMap::new();
    let mut queue: Vec<((usize, usize), usize)> = Vec::new();

    let max_x: usize = heatmap[0].len() - 1;
    let max_y: usize = heatmap.len() - 1;

    queue.push(((0, 0), heatmap[0][0]));

    while !queue.is_empty() {
        let item: ((usize, usize), usize) = queue.pop().unwrap();

        if visited_blocks.contains_key(&item.0) {
            if *visited_blocks.get(&item.0).unwrap() <= item.1 {
                continue;
            }
        }

        visited_blocks.insert(item.0, item.1);

        if 0 < item.0.0 {
            let new_tile: (usize, usize) = (item.0.0 - 1, item.0.1);
            queue.push((new_tile, item.1 + heatmap[new_tile.0][new_tile.1]));
        }

        if 0 < item.0.1 {
            let new_tile: (usize, usize) = (item.0.0, item.0.1 - 1);
            queue.push((new_tile, item.1 + heatmap[new_tile.0][new_tile.1]));
        }
        if item.0.0 < max_y {
            let new_tile: (usize, usize) = (item.0.0 + 1, item.0.1);
            queue.push((new_tile, item.1 + heatmap[new_tile.0][new_tile.1]));
        }
        if item.0.1 < max_x {
            let new_tile: (usize, usize) = (item.0.0, item.0.1 + 1);
            queue.push((new_tile, item.1 + heatmap[new_tile.0][new_tile.1]));
        }
    }

    for i in 0..max_y + 1 { 
        for j in 0..max_x + 1 {
            print!("{:?} ", visited_blocks.get(&(i, j)).unwrap());
        }
        println!();
    }

    // *visited_blocks.get(&(max_y, max_x)).unwrap()

    1
}
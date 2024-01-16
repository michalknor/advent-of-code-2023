use std::fs::File;
use std::io::Read;

use std::cmp;
use std::collections::HashMap;
use std::collections::HashSet;


pub fn main(filename: &str) -> String {
    let mut file = File::open(filename).expect("Failed to open file");
	let mut file_content: String = String::new();

	file.read_to_string(&mut file_content).expect("Failed to read file content");

	let bricks: Vec<((u16, u16, u16), (u16, u16, u16))> = file_content
        .lines()
        .map(|line| {
            let (start, end) = line
				.split_once("~")
				.unwrap();
			(format_coords(start), format_coords(end))
			}
		)
        .collect();
	
	solve(&bricks).to_string()
}


fn format_coords(string: &str) -> (u16, u16, u16) {
	let coords: Vec<u16> = string
		.split(",")
		.map(|coord| 
			coord
				.parse::<u16>()
				.unwrap()
		)
		.collect();

	(coords[0], coords[1], coords[2] - 1)
}


fn solve(bricks: &Vec<((u16, u16, u16), (u16, u16, u16))>) -> usize {
	let max_coords: (u16, u16, u16) = bricks
		.iter()
		.fold((0, 0, 0), |max_coords, coords|
			(
				cmp::max(cmp::max(coords.0.0, coords.1.0), max_coords.0), 
				cmp::max(cmp::max(coords.0.1, coords.1.1), max_coords.1), 
				cmp::max(cmp::max(coords.0.2, coords.1.2), max_coords.2)
			)
		);

    let mut space: Vec<Vec<Vec<u16>>> = vec![
		vec![
			vec![
				u16::MIN; (max_coords.2 + 1) as usize
			]; (max_coords.1 + 1) as usize
		]; (max_coords.0 + 1) as usize
	];

	let mut bricks = bricks.clone();
	bricks.sort_unstable_by_key(|b| b.1.2);

    let mut bricks_below: HashMap<u16, HashSet<u16>> = HashMap::new();

	for (i, brick) in bricks.iter().enumerate() {
		let mut offset_z: u16 = 0;
		'outer: for _ in 0..brick.0.2 {
			for x in brick.0.0..brick.1.0+1 {
				for y in brick.0.1..brick.1.1+1 {
					if space[x as usize][y as usize][(brick.0.2-offset_z-1) as usize] != u16::MIN {
						break 'outer;
					}
				}	
			}
			offset_z += 1;
		}

		for x in brick.0.0..brick.1.0+1 {
			for y in brick.0.1..brick.1.1+1 {
				for z in brick.0.2-offset_z..brick.1.2-offset_z+1 {
					space[x as usize][y as usize][z as usize] = (i+1) as u16;

					if z == u16::MIN {
						continue;
					}

					let tile_below: u16 = space[x as usize][y as usize][(z-1) as usize];

					if tile_below != (i+1) as u16 && tile_below != u16::MIN {
						bricks_below.entry(tile_below).or_insert(HashSet::from([(i+1) as u16])).insert((i+1) as u16);
					}
				}
			}	
		}
	}

	get_sum_of_other_bricks_that_would_fall(&bricks_below, bricks.len())
}


fn get_sum_of_other_bricks_that_would_fall(bricks_below: &HashMap<u16, HashSet<u16>>, number_of_bricks: usize) -> usize {
    let mut sum_of_other_bricks_that_would_fall: usize = 0;

    for i in 1..number_of_bricks+1 {
        let mut bricks_below_removable: HashMap<u16, HashSet<u16>> = bricks_below.clone();
        let mut stack: Vec<u16> = vec![i as u16];

        while !stack.is_empty() {
            let removed_brick: u16 = stack.pop().unwrap();

            if !bricks_below_removable.contains_key(&removed_brick) {
                continue;
            }

            for brick in bricks_below_removable.get(&removed_brick).unwrap() {
                if n_of_support(&bricks_below_removable, *brick) == 1 {
                    stack.push(*brick);
                    sum_of_other_bricks_that_would_fall += 1;
                }
            }

            bricks_below_removable.remove(&removed_brick);
        }
    }

    sum_of_other_bricks_that_would_fall
}


fn n_of_support(bricks_below_removable: &HashMap<u16, HashSet<u16>>, brick_to_count: u16) -> u16 {
    let mut n_of_support: u16 = 0;

    for brick in bricks_below_removable {
        for brick2 in brick.1 {
            if brick_to_count == *brick2 {
                n_of_support += 1;
            }
        }
    }

    n_of_support
}
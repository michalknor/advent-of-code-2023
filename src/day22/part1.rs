use std::fs::File;
use std::io::Read;

use std::cmp;
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
	
	let mut can_be_removed: Vec<bool> = vec![true; bricks.len()];

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

		let mut bricks_below: HashSet<u16> = HashSet::new();

		for x in brick.0.0..brick.1.0+1 {
			for y in brick.0.1..brick.1.1+1 {
				for z in brick.0.2-offset_z..brick.1.2-offset_z+1 {
					space[x as usize][y as usize][z as usize] = (i+1) as u16;

					if z == u16::MIN {
						continue;
					}

					let tile_below: u16 = space[x as usize][y as usize][(z-1) as usize];

					if tile_below != (i+1) as u16 && tile_below != u16::MIN {
						bricks_below.insert(tile_below);
					}
				}
			}	
		}

		if bricks_below.len() == 1 {
			can_be_removed[*bricks_below.iter().next().unwrap() as usize] = false;
		}
	}

	can_be_removed.iter().filter(|possible| **possible).count()
}
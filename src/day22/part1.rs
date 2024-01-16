use std::fs::File;
use std::io::Read;

use std::cmp;


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

	(coords[0], coords[1], coords[2])
}


fn solve(bricks: &Vec<((u16, u16, u16), (u16, u16, u16))>) -> usize {
	let space: Vec<Vec<Vec<u16>>> = bricks_to_space(&bricks);
	let mut n_of_removable_bricks = 0;

	for it in space {
		for it2 in it {
			for it3 in it2 {
				print!("{}", it3);
			}
			println!()
		}
		println!()
	}

	0
}


fn bricks_to_space(bricks: &Vec<((u16, u16, u16), (u16, u16, u16))>) -> Vec<Vec<Vec<u16>>> {
    let max_coords: (u16, u16, u16) = bricks
		.iter()
		.fold((0, 0, 0), |max_coords, coords|
			(
				cmp::max(cmp::max(coords.0.0, coords.1.0), max_coords.0), 
				cmp::max(cmp::max(coords.0.1, coords.1.1), max_coords.1), 
				cmp::max(cmp::max(coords.0.2 - 1, coords.1.2 - 1), max_coords.2)
			)
		);

    let mut space: Vec<Vec<Vec<u16>>> = vec![
		vec![
			vec![
				u16::MIN; (max_coords.2 + 1) as usize
			]; (max_coords.1 + 1) as usize
		]; (max_coords.0 + 1) as usize
	];

	let mut bricks2 = bricks.clone();

	bricks2.sort_unstable_by_key(|b| b.1); 

	for (i, brick) in bricks2.iter().enumerate() {
		for x in brick.0.0..brick.1.0+1 {
			for y in brick.0.1..brick.1.1+1 {
				for z in brick.0.2..brick.1.2+1 {
					space[x as usize][y as usize][(z-1) as usize] = (i+1) as u16;
				}
			}	
		}
	}

	space
}
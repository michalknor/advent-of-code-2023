use std::fs::File;
use std::io::Read;


const BOUNDARIES: (f64, f64) = (200000000000000.0, 400000000000000.0);


pub fn main(filename: &str) -> String {
    let mut file = File::open(filename).expect("Failed to open file");
	let mut file_content: String = String::new();

	file.read_to_string(&mut file_content).expect("Failed to read file content");

	let hailstones: Vec<((i64, i64, i64), (i64, i64, i64))> = file_content
        .lines()
        .map(|line| {
            let (start, end) = line
				.split_once(" @ ")
				.unwrap();
			(format_coords(start), format_coords(end))
			}
		)
        .collect();
	
	solve(&hailstones).to_string()
}


fn format_coords(string: &str) -> (i64, i64, i64) {
	let coords: Vec<i64> = string
		.split(", ")
		.map(|coord| 
			coord
				.parse::<i64>()
				.unwrap()
		)
		.collect();

	(coords[0], coords[1], coords[2])
}


fn solve(hailstones: &Vec<((i64, i64, i64), (i64, i64, i64))>) -> usize {
	let linear_functions: Vec<(f64, f64, f64)> = hailstones_to_functions(&hailstones);

	let mut count: usize = 0;

	for (i, lf1) in linear_functions.iter().enumerate() {
		for (j, lf2) in linear_functions.iter().enumerate() {
			if i >= j {
				continue;
			}

			let tmp: f64 = lf1.0 * lf2.1 - lf2.0 * lf1.1;
			
			let intersection: (f64, f64) = (
				(lf1.1 * lf2.2 - lf2.1 * lf1.2) / tmp, 
				(lf1.2 * lf2.0 - lf2.2 * lf1.0) / tmp
			);

			if is_point_in_the_past(&hailstones[i], &intersection) || is_point_in_the_past(&hailstones[j], &intersection) {
				continue;
			}

			if (BOUNDARIES.0 <= intersection.0 && intersection.0 <= BOUNDARIES.1) && (BOUNDARIES.0 <= intersection.1 && intersection.1 <= BOUNDARIES.1) {
				count += 1;
			}
		}
	}

    count
}


fn hailstones_to_functions(hailstones: &Vec<((i64, i64, i64), (i64, i64, i64))>) -> Vec<(f64, f64, f64)> {
	let mut functions: Vec<(f64, f64, f64)> = vec![(0.0, 0.0, 0.0); hailstones.len()];

	for (i, hailstone) in hailstones.iter().enumerate() {
		let multiplicate: f64 = hailstone.0.0 as f64 / hailstone.1.0 as f64;
		let a: f64 = hailstone.1.1 as f64 / hailstone.1.0 as f64;
		let c: f64 = hailstone.0.1 as f64 - (hailstone.1.1 as f64 * multiplicate);

		functions[i] = (a, -1.0, c);
	}

    functions
}


fn is_point_in_the_past(hailstone: &((i64, i64, i64), (i64, i64, i64)), intersection: &(f64, f64)) -> bool {
	match hailstone.1.0 {
		n if n > 0 => {
			if (hailstone.0.0 as f64) > intersection.0 {
				return true;
			}
			false
		},
		n if n < 0 => {
			if (hailstone.0.0 as f64) < intersection.0 {
				return true;
			}
			false
		}
		_ => {
			if (hailstone.0.0 as f64) != intersection.0 {
				return true;
			}
			false
		}
	}
}
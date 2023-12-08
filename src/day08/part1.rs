use std::collections::HashMap;


const DESTINATION: &str = "ZZZ";


fn get_hand_strength(hand: &str) -> bool {
    let label_counts: Vec<usize> = hand
		    .chars()
		    .collect::<Vec<char>>()
		    .iter()
		    .map(|&label| hand
			    .chars()
			    .filter(|&c| c == label)
			    .count())
		    .filter(|&c| c != 1)
		    .collect();

	false
}


pub fn main(testing: bool) {
	let file_content: &str = if testing {
		include_str!("test.txt")
	}
	else {
		include_str!("input.txt")
	};

	let file_content = &(file_content
		.replace('=', "")
		.replace('(', "")
		.replace(',', "")
		.replace(')', ""));

	let result: Vec<&str> = file_content
		.split_whitespace()
		.skip(1)
		.collect();

		println!("{:?}", result);
	// let result: HashMap<_, _> = file_content
	// 	.split_whitespace()
	// 	.skip(2)
	// 	.map(|line| {
	// 		let mut parts = line.splitn(2, " = ");
	// 		let key = parts.next().unwrap();
	// 		let value = parts.next().unwrap();
	// 		let vec_values: Vec<&str> = value[1..value.len() - 1].split(", ").collect();
	// 		(key.to_string(), vec_values)
    // 	}).collect();

	// let result: HashMap<_, _> = result.chunks(3).map(|chunk| {
    //     let key = chunk[0];
    //     let value = chunk[2];
    //     let vec_values: Vec<_> = value[1..value.len() - 1].split(", ").collect();
    //     (key.to_string(), vec_values)
    // }).collect();
}

//250474325
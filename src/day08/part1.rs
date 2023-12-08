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

	let result: Vec<&str> = file_content
		.split_whitespace()
		.skip(1)
		.collect();

	let result: HashMap<_, _> = file_content
		.split_whitespace()
		.skip(1)
		.map(|line| {
			let mut parts = line.splitn(2, " = ");
			let key = parts.next().unwrap();
			let value = parts.next().unwrap();
			let vec_values: Vec<&str> = value[1..value.len() - 1].split(", ").collect();
			(key.to_string(), vec_values)
    	}).collect();

	println!("{:?}", result);
}

//250474325
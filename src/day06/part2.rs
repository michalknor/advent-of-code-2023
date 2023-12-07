fn number_of_ways_to_win_a_race(time: u64, distance: u64) -> u64 {
	let mut result = 0;

	for i in 1..time-1 {
		if i * (time-i) > distance {
			result += 1;
		}
	}
	
	result
}


pub fn main(testing: bool) {
	let file_content = if testing {
		include_str!("test.txt")
	}
	else {
		include_str!("input.txt")
	};

	let times: u64 = file_content
		.lines()
		.next()
		.unwrap()
		.split_whitespace()
		.skip(1)
		.collect::<String>()
		.parse()
    	.unwrap();

	let distance: u64 = file_content
		.lines()
		.nth(1)
		.unwrap()
		.split_whitespace()
		.skip(1)
		.collect::<String>()
		.parse()
    	.unwrap();

	println!("{}", number_of_ways_to_win_a_race(times, distance));
}
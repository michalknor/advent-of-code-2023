fn number_of_ways_to_win_a_race(time: u32, distance: u32) -> u32 {
	let mut result = 0;

	for i in 1..time-1 {
		if i * (time-i) > distance {
			result += 1;
		}
	}
	
	result
}


fn number_of_ways_multiplied_together(times: Vec<u32>, distances: Vec<u32>) -> u32 {
	let mut result = 1;

	for (i, time) in times.iter().enumerate() {
		result *= number_of_ways_to_win_a_race(*time, distances[i]);
	}

	result
}


pub fn main(testing: bool) {
	let file_content: &str = if testing {
		include_str!("test.txt")
	}
	else {
		include_str!("input.txt")
	};

	let times: Vec<u32> = file_content
		.lines()
		.next()
		.unwrap()
		.split(' ')
		.skip(2)
		.filter(|&s| !s.is_empty())
		.map(|s| s.parse::<u32>().unwrap())
		.collect();

	let distances: Vec<u32> = file_content
		.lines()
		.nth(1)
		.unwrap()
		.split(' ')
		.skip(2)
		.filter(|&s| !s.is_empty())
		.map(|s| s.parse::<u32>().unwrap())
		.collect();

	println!("{}", number_of_ways_multiplied_together(times, distances));
}
pub fn main(testing: bool) {
	let file_content: &str;
	if testing {
		file_content = include_str!("test.txt");
	}
	else {
		file_content = include_str!("input.txt");
	}

	let times: Vec<u32> = file_content
		.lines()
		.next()
		.unwrap()
		.split(" ")
		.skip(2)
		.filter(|&s| !s.is_empty())
		.map(|s| s.parse::<u32>().unwrap())
		.collect();

	let distances: Vec<u32> = file_content
		.lines()
		.skip(1)
		.next()
		.unwrap()
		.split(" ")
		.skip(2)
		.filter(|&s| !s.is_empty())
		.map(|s| s.parse::<u32>().unwrap())
		.collect();

	println!("{:?}", times);
	println!("{:?}", distances);
}
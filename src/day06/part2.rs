pub fn main(testing: bool) {
	let file_content: &str;
	if testing {
		file_content = include_str!("test.txt");
	}
	else {
		file_content = include_str!("input.txt");
	}

	let lines: Vec<&str> = file_content.lines().collect();
	
    let data: Vec<_> = file_content
		.split(" ")
        .collect();

	println!("{:?}", data);
}
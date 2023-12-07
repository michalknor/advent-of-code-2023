pub fn main(testing: bool) {
	let file_content: &str;
	if testing {
		file_content = include_str!("test.txt");
	}
	else {
		file_content = include_str!("input.txt");
	}

	// let cards: Vec<Vec<&str>> = file_content
	// 	.split_whitespace()
	// 	.collect()
	// 	.chunks_exact(2).map(|chunk| [chunk[0], chunk[1]]).collect();;


	let input_array = ["32T3K", "765", "T55J5", "684", "KK677", "28", "KTJJT", "220", "QQQJA", "483"];

    // Convert the 1D array into a 2D array with a size of 2 in one line
    let cards: Vec<[&str; 2]> = input_array.chunks_exact(2).map(|chunk| [chunk[0], chunk[1]]).collect();

    // Print the resulting 2D array

	println!("{:?}", cards);
}
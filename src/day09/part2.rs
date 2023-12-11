fn sum_extrapolated_values(histories: &Vec<Vec<i32>>) -> i32 {
	let mut sum_extrapolated_values: i32 = 0;

	for history in histories {
		let mut current_history: Vec<i32> = history.clone();
		current_history.reverse();

		loop {
			let mut all_zeroes = true;

			for i in 0..current_history.len() - 1 {
				current_history[i] = current_history[i+1] - current_history[i];
				if current_history[i] != 0 {
					all_zeroes = false;
				}
			}

			sum_extrapolated_values += current_history.pop().unwrap();

			if all_zeroes {
				break;
			}
		}
	}

	sum_extrapolated_values
}


pub fn main(testing: bool) {
	let file_content: &str = if testing {
		include_str!("test.txt")
	}
	else {
		include_str!("input.txt")
	};

	let histories: Vec<Vec<i32>> = file_content
		.lines()
		.map(|line| line
            .split_whitespace()
            .map(|s| s.parse::<i32>().unwrap())
            .collect())
		.collect();

	println!("{:?}", sum_extrapolated_values(&histories));
}
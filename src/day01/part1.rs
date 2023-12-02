pub fn main() -> std::io::Result<()> {
	let file_content: &str = include_str!("input.txt");

	let mut result: u32 = 0;
	for line in file_content.lines() {
		let (mut first_digit, mut last_digit): (u32, u32) = (0, 0);

		for character in line.chars() {
			if character.is_digit(10) {
				first_digit = character.to_digit(10).unwrap_or_default() as u32;
				break;
			}
		}
		
		for character in line.chars().rev() {
			if character.is_digit(10) {
				last_digit = character.to_digit(10).unwrap_or_default() as u32;
				break;
			}
		}

		result += first_digit*10 + last_digit;
    }

	println!("{}", result);

    Ok(())
}
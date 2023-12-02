fn get_digit(string_so_far: &str) -> Option<u32> {
	match string_so_far {
		s if s.contains("one") => {
			Some(1)
		}
		s if s.contains("two") => {
			Some(2)
		}
		s if s.contains("three") => {
			Some(3)
		}
		s if s.contains("four") => {
			Some(4)
		}
		s if s.contains("five") => {
			Some(5)
		}
		s if s.contains("six") => {
			Some(6)
		}
		s if s.contains("seven") => {
			Some(7)
		}
		s if s.contains("eight") => {
			Some(8)
		}
		s if s.contains("nine") => {
			Some(9)
		}
		_ => None,
	}
}

pub fn main() -> std::io::Result<()> {
	let file_content: &str = include_str!("input.txt");

	let mut result: u32 = 0;
	for line in file_content.lines() {
		let (mut first_digit, mut last_digit): (u32, u32) = (0, 0);
		let mut string_so_far = String::new();

		for character in line.chars() {
			if character.is_digit(10) {
				first_digit = character.to_digit(10).unwrap_or_default() as u32;
				break;
			}

			string_so_far.push_str(&character.to_string());
			match get_digit(&string_so_far) {
				Some(digit) => {
					first_digit = digit;
					break;
				}
				None => {
				}
			};
		}

		string_so_far = String::new();
		
		for character in line.chars().rev() {
			if character.is_digit(10) {
				last_digit = character.to_digit(10).unwrap_or_default() as u32;
				break;
			}

			string_so_far = character.to_string() + &string_so_far;
			match get_digit(&string_so_far) {
				Some(digit) => {
					last_digit = digit;
					break;
				}
				None => {
				}
			};
		}

		result += first_digit*10 + last_digit;
    }

	println!("{}", result);

    Ok(())
}
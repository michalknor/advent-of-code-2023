use std::fs::File;
use std::io::Read;


pub fn main(filename: &str) -> String {
    let mut file: File = File::open(filename).expect("Failed to open file");
<<<<<<< HEAD
	let mut file_content: String = String::new();
=======
	let mut file: File_content: String = String::new();
>>>>>>> 84da3640cab7e9c156fa7aa9bc01b57a9bcc4c39

	file.read_to_string(&mut file_content).expect("Failed to read file content");

    let platform: Vec<Vec<char>> = file_content
		.lines()
		.map(|line| line
			.chars()
			.collect())
		.collect();
        
    sum_of_load_on_the_north_support_beams(&platform).to_string()
}


fn sum_of_load_on_the_north_support_beams(platform: &Vec<Vec<char>>) -> usize {
    let platform_on_north_support_beams = tilt_on_north(&platform);

    platform_on_north_support_beams
        .iter()
        .enumerate()
        .map(|(i, row)| 
            row
                .iter()
                .filter(|&&c| c == 'O')
                .count() * (platform_on_north_support_beams.len() - i)
        )
        .sum()
}


fn tilt_on_north(platform: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut platform_on_north_support_beams = platform.clone();

    for j in 0..platform[0].len() {
        let mut new_i_index: usize = 0;
        for i in 0..platform.len() {
            match platform[i][j] {
                'O' => {
                    platform_on_north_support_beams[i][j] = '.';
                    platform_on_north_support_beams[new_i_index][j] = 'O';
                    new_i_index += 1;
                },
                '#' => {
                    new_i_index = i + 1;
                }
                _ => continue
            }
        }
    }

    platform_on_north_support_beams
}
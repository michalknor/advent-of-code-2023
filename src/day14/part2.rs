use std::fs::File;
use std::io::Read;

use std::collections::HashMap;


const NUMBER_OF_CYCLES: usize = 1_000_000_000;


pub fn main(filename: &str) -> String {
    let mut file = File::open(filename).expect("Failed to open file");
	let mut file_content: String = String::new();

	file.read_to_string(&mut file_content).expect("Failed to read file content");

    let platform: Vec<Vec<char>> = file_content
		.lines()
		.map(|line| line
			.chars()
			.collect())
		.collect();
        
    sum_of_load_after_cycles(&platform).to_string()
}


fn sum_of_load_after_cycles(platform: &Vec<Vec<char>>) -> usize {
    let mut platform_after_cycle = platform.clone();
    let mut seen: HashMap<Vec<Vec<char>>, usize>= HashMap::new();

    for i in 0..NUMBER_OF_CYCLES {
        if let Some(prev) = seen.get(&platform_after_cycle) {
            if (NUMBER_OF_CYCLES - i) % (i - prev) == 0 {
                return calculate_score(&platform_after_cycle);
            }
        }

        seen.insert(platform_after_cycle.clone(), i);

        platform_after_cycle = cycle(&platform_after_cycle);
    }

    calculate_score(&platform_after_cycle)
}


fn cycle(platform: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    tilt_on_east(&tilt_on_south(&tilt_on_west(&tilt_on_north(&platform))))
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


fn tilt_on_west(platform: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut platform_on_north_support_beams = platform.clone();

    for i in 0..platform.len() {
        let mut new_j_index: usize = 0;
        for j in 0..platform[0].len() {
            match platform[i][j] {
                'O' => {
                    platform_on_north_support_beams[i][j] = '.';
                    platform_on_north_support_beams[i][new_j_index] = 'O';
                    new_j_index += 1;
                },
                '#' => {
                    new_j_index = j + 1;
                }
                _ => continue
            }
        }
    }

    platform_on_north_support_beams
}


fn tilt_on_south(platform: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut platform_on_north_support_beams = platform.clone();

    for j in 0..platform[0].len() {
        let mut new_i_index: usize = platform.len();
        for i in (0..platform.len()).rev() {
            match platform[i][j] {
                'O' => {
                    platform_on_north_support_beams[i][j] = '.';
                    platform_on_north_support_beams[new_i_index - 1][j] = 'O';
                    new_i_index -= 1;
                },
                '#' => {
                    new_i_index = i;
                }
                _ => continue
            }
        }
    }

    platform_on_north_support_beams
}


fn tilt_on_east(platform: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut platform_on_north_support_beams = platform.clone();

    for i in 0..platform.len() {
        let mut new_j_index: usize = platform[0].len();
        for j in (0..platform[0].len()).rev() {
            match platform[i][j] {
                'O' => {
                    platform_on_north_support_beams[i][j] = '.';
                    platform_on_north_support_beams[i][new_j_index - 1] = 'O';
                    new_j_index -= 1;
                },
                '#' => {
                    new_j_index = j;
                }
                _ => continue
            }
        }
    }

    platform_on_north_support_beams
}


fn calculate_score(platform: &Vec<Vec<char>>) -> usize {
    platform
        .iter()
        .enumerate()
        .map(|(i, row)| 
            row
                .iter()
                .filter(|&&c| c == 'O')
                .count() * (platform.len() - i)
        )
        .sum()
}
use std::fs::File;
use std::io::Read;


pub fn main(filename: &str) -> String {
    let mut file = File::open(filename).expect("Failed to open file");
	let mut file_content: String = String::new();

	file.read_to_string(&mut file_content).expect("Failed to read file content");

    let mirrors: Vec<Vec<Vec<char>>> = file_content
        .lines()
        .collect::<Vec<&str>>()
        .split(|mirror| mirror.is_empty())
        .map(|mirror| 
            mirror
                .iter()
                .map(|line| 
                    line
                        .chars()
                        .collect()
                )
                .collect()
        )
        .collect();

    get_sum_of_notes(&mirrors).to_string()
}


fn get_sum_of_notes(mirrors: &Vec<Vec<Vec<char>>>) -> usize {
    let mut sum: usize = 0;
    for mirror in mirrors {
        sum += get_vertical_reflection_line_number(&mirror);
        sum += get_horizontal_reflection_line_number(&mirror) * 100;
    }

    sum
}


fn get_horizontal_reflection_line_number(mirror: &Vec<Vec<char>>) -> usize {
    for pivot in 1..mirror.len()/2 + mirror.len()%2 {
        let mut mirrored = true;
        for i in 0..pivot {
            if mirror[pivot-i-1] != mirror[pivot+i] {
                mirrored = false;
                break;
            }
        }

        if mirrored {
            return pivot
        }
    }

    for pivot in mirror.len()/2 + mirror.len()%2..mirror.len() {
        let mut mirrored = true;
        for i in 0..mirror.len()-pivot {
            if mirror[pivot-i-1] != mirror[pivot+i] {
                mirrored = false;
                break;
            }
        }

        if mirrored {
            return pivot
        }
    }

    0
}


fn get_vertical_reflection_line_number(mirror: &Vec<Vec<char>>) -> usize {
    let mut flipped_mirror: Vec<Vec<char>> = Vec::new();

    for i in 0..mirror[0].len() {
        flipped_mirror.push(Vec::new());
        for j in 0..mirror.len() {
            flipped_mirror[i].push(mirror[j][i]);
        }
    }

    get_horizontal_reflection_line_number(&flipped_mirror)
}
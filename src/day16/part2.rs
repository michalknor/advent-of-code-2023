use std::fs::File;
use std::io::Read;

use std::collections::HashMap;


pub fn main(filename: &str) -> String {
    let mut file = File::open(filename).expect("Failed to open file");
	let mut file_content: String = String::new();

	file.read_to_string(&mut file_content).expect("Failed to read file content");

    let sequences: Vec<Vec<char>> = file_content
        .split(',')
        .map(|s| s.chars().collect())
        .collect();

    get_sum_of_sequences(&sequences).to_string()
}


fn get_sum_of_sequences(sequences: &Vec<Vec<char>>) -> usize {
    let mut sum: usize = 0;
    let mut boxes: [HashMap<String, (usize, usize)>; 256] = [(); 256].map(|_| HashMap::default());

    for (i, sequence) in sequences.iter().enumerate() {
        let mut sequence_sum: usize = 0;
        for j in 0..sequence.len() {
            if sequence[j] == '-' {
                let key: String = sequence[0..j].iter().collect::<String>();
                boxes[sequence_sum].remove(key.clone().as_str());

                break;
            }

            if sequence[j] == '=' {
                let key: String = sequence[0..j].iter().collect::<String>();
                let new_value: usize = sequence[j+1].to_digit(10).unwrap().try_into().unwrap();
                boxes[sequence_sum]
                    .entry(key)
                    .and_modify(|(value, _)| *value = new_value)
                    .or_insert((new_value, i));
                
                break;
            }

            sequence_sum = (sequence_sum + sequence[j] as usize) * 17 % 256;
        }
    }

    for (i, bandbox) in boxes.iter().enumerate() {
        if bandbox.is_empty() {
            continue;
        }
        
        let mut vec_of_tuples: Vec<(usize, usize)> = bandbox
            .into_iter()
            .map(|(_, v)| *v)
            .collect();
        vec_of_tuples.sort_by_key(|&(_, u8_value)| u8_value);

        for j in 0..vec_of_tuples.len() {
            sum += (i + 1) * (j + 1) * vec_of_tuples[j].0;
        }
    }

    sum
}
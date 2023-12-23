use std::{thread, time};
use std::fs::File;
use std::io::Read;
use std::cmp;


pub fn main(filename: &str) -> String {
    let mut file = File::open(filename).expect("Failed to open file");
	let mut file_content: String = String::new();

	file.read_to_string(&mut file_content).expect("Failed to read file content");

    let springs: Vec<(&str, Vec<usize>)> = file_content
   		.lines()
        .map(|s| {
            let mut parts = s.split_whitespace();
            let str_part = parts.next().unwrap();
            let vec_part: Vec<usize> = parts
                .next()
                .unwrap()
                .split(',')
                .map(|num| num.parse().unwrap())
                .collect();
            (str_part, vec_part)
        })
        .collect();

    get_number_of_arrangements(&springs).to_string()
}


fn get_number_of_arrangements(springs: &Vec<(&str, Vec<usize>)>) -> usize {
    let mut number_of_arrangements: usize = 0;

    for spring in springs {
        let mut number_of_arrangements_parts: [usize; 2] = [0, 0];
        println!("{:?}", spring);
        for i in 0..2 {
            let legend: Vec<usize> = spring
                .1
                .iter()
                .cloned()
                .cycle()
                .take((i+1) * spring.1.len())
                .collect();

            let total: usize = legend.iter().sum();
            // println!("{:?}", legend);
            // println!("{number_of_arrangements}");
            let mut queue: Vec<(String, usize, usize)> = vec![
                (
                    vec![spring.0.to_string(); i+1].join("?"), 
                    spring.0.chars().filter(|&c| c == '#').count() * (i+1), 
                    spring.0.chars().filter(|&c| c == '?').count() * (i+1) + i
                )
            ];
            // println!("{:?}", queue);
            while !queue.is_empty() {
                let item: (String, usize, usize) = queue.pop().unwrap();
                let item_spring: String = item.0;
                let item_fill_count: usize = item.1;
                let item_unknown_count: usize = item.2;
                // println!("{:?}, {}, {}", item_spring, item_fill_count, item_unknown_count);
                // println!("{:?}", queue.len());
                // thread::sleep(time::Duration::from_millis(100));
                if total > item_unknown_count + item_fill_count {
                    continue;
                }
                if total == item_fill_count {
                    if arrangement_is_valid(&item_spring, &legend) {
                        number_of_arrangements_parts[i] += 1;
                        // println!("{number_of_arrangements}");
                    }
                    continue
                }
                queue.push((item_spring.replacen("?", ".", 1), item_fill_count, item_unknown_count - 1));
                queue.push((item_spring.replacen("?", "#", 1), item_fill_count + 1, item_unknown_count - 1));
            }
            // println!("asdasdasd{}", number_of_arrangements_parts[i]);
        }
        number_of_arrangements += cmp::max(number_of_arrangements_parts[0] * (number_of_arrangements_parts[1] / number_of_arrangements_parts[0]).pow(4), number_of_arrangements_parts[0]);
    }

    number_of_arrangements
}


fn arrangement_is_valid(item: &String, legend: &Vec<usize>) -> bool {
    let mut result: Vec<usize> = Vec::new();
    let mut count: usize = 0;

    for c in item.chars() {
        match c {
            '#' => count += 1,
            _ => {
                if count > 0 {
                    result.push(count);
                    count = 0;
                }
            }
        }
    }

    if count > 0 {
        result.push(count);
    }

    &result == legend
}
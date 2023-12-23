use std::{thread, time};
use std::fs::File;
use std::io::Read;


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
        let legend: Vec<usize> = spring
			.1
			.iter()
			.cloned()
			.cycle()
			.take(5 * spring.1.len())
			.collect();
        let total: usize = legend.iter().sum();
        let mut queue: Vec<(String, usize, usize)> = vec![
			(
				vec![spring.0.to_string(); 5].join("?"), 
				spring.0.chars().filter(|&c| c == '#').count() * 5, 
				spring.0.chars().filter(|&c| c == '?').count() * 5 + 4
			)
		];
		println!("{:?}", legend);
		println!("{:?}", queue);
		println!("{:?}", total);
		println!("{number_of_arrangements}");
        while !queue.is_empty() {
            let item: (String, usize, usize) = queue.pop().unwrap();
            let item_spring: String = item.0;
			// println!("{:?}", item_spring);
			// println!("{:?}", queue.len());
			// thread::sleep(time::Duration::from_millis(100));
            let item_fill_count: usize = item.1;
            let item_unknown_count: usize = item.2;
            if total > item_unknown_count + item_fill_count {
                continue;
            }
            if total == item_fill_count {
                if arrangement_is_valid(&item_spring, &legend) {
                    number_of_arrangements += 1;
					println!("{number_of_arrangements}");
                }
                continue
            }
            queue.push((item_spring.replacen("?", ".", 1), item_fill_count, item_unknown_count - 1));
            queue.push((item_spring.replacen("?", "#", 1), item_fill_count + 1, item_unknown_count - 1));
        }
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
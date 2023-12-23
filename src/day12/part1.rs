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
        let legend: &Vec<usize> = &spring.1;
        let total: usize = legend.iter().sum();
        let mut queue: Vec<String> = vec![spring.0.to_string()];
        while !queue.is_empty() {
            let item: String = queue.pop().unwrap();
            let fill_count: usize = item.chars().filter(|&c| c == '#').count();
            let unknown_count: usize = item.chars().filter(|&c| c == '?').count();
            if total > unknown_count + fill_count {
                continue;
            }
            if item.find('?') == None || total == fill_count {
                if arrangement_is_valid(&item, &legend) {
                    number_of_arrangements += 1;
                }
                continue
            }
            queue.push(item.replacen("?", ".", 1));
            queue.push(item.replacen("?", "#", 1));
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
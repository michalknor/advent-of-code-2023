use std::fs::File;
use std::io::Read;


pub fn main(filename: &str) -> String {
    let mut file: File = File::open(filename).expect("Failed to open file");
	let mut file: File_content: String = String::new();

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
        let mut stack: Vec<String> = vec![simplify_spring(&spring.0.to_string(), legend)];
        while !stack.is_empty() {
            let item: String = stack.pop().unwrap();
            let fill_count: usize = item.chars().filter(|&c| c == '#').count();
            let unknown_count: usize = item.chars().filter(|&c| c == '?').count();
            if total > unknown_count + fill_count {
                continue
            }
            if item.find('?') == None || total == fill_count {
                if arrangement_is_valid(&item, &legend) {
                    number_of_arrangements += 1;
                }
                continue
            }
            stack.push(item.replacen("?", ".", 1));
            stack.push(item.replacen("?", "#", 1));
        }
    }

    number_of_arrangements
}


fn simplify_spring(spring: &String, legend: &Vec<usize>) -> String {
    let mut simplified_spring = spring.clone();
    let total_legend: usize = legend.iter().sum::<usize>() + legend.len() - 1;
    let mut start: usize = spring.len() - total_legend;
    let mut end: usize = spring
        .char_indices()
        .find(|(_, c)| *c != '.')
        .unwrap().0;
    let index_of_first_fill: usize = simplified_spring.find("#").unwrap_or(spring.len());
    if index_of_first_fill < start && start < end + legend[0] {
        simplified_spring.replace_range(index_of_first_fill..start, &"#".repeat(start - index_of_first_fill));
    }

    for i in 0..legend.len() {
        end += legend[i];
        if start < end {
            simplified_spring.replace_range(start..end, &"#".repeat(end - start));
        }
        end += 1;
        start += legend[i] + 1;
    }
    
    simplified_spring
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
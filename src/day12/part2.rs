use std::fs::File;
use std::io::Read;

use rayon::prelude::*;

use std::collections::HashMap;


const EMPTY_STRING: String = String::new();
const REPEAT_TIMES: usize = 5;


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
    let mut memory: HashMap<(String, Vec<usize>), usize> = HashMap::new();

    for spring in springs {
        println!("{:?}", spring);
        let legend: Vec<usize> = spring
            .1
            .iter()
            .cloned()
            .cycle()
            .take((REPEAT_TIMES) * spring.1.len())
            .collect();

        let total: usize = legend.iter().sum();
        
        let mut queue: Vec<(String, usize, Vec<usize>, Vec<(String, Vec<usize>)>)> = vec![
            (
                simplify_spring(&vec![spring.0.to_string(); REPEAT_TIMES].join("?"), &legend.clone()),
                0,
                legend.clone(),
                Vec::new()
            )
        ];

        // println!("{:?}", simplify_spring(&vec![spring.0.to_string(); REPEAT_TIMES].join("?"), &legend));

        queue[0].1 = queue[0].0.chars().filter(|&c| c == '#').count();
        queue[0].3.push((simplify_spring(&vec![spring.0.to_string(); REPEAT_TIMES].join("?"), &legend), legend.clone()));

        while !queue.is_empty() {
            let item: (String, usize, Vec<usize>, Vec<(String, Vec<usize>)>) = queue.pop().unwrap();
            let item_spring: String = item.0;
            let fill_count: usize = item.1;
            let item_legend: Vec<usize> = item.2;
            let item_history: Vec<(String, Vec<usize>)> = item.3;

            if memory.contains_key(&(item_spring.clone(), item_legend.clone())) {
                let memory_new_spring: usize = *memory.get(&(item_spring.clone(), item_legend.clone())).unwrap();
                number_of_arrangements += memory_new_spring;
                let mut bla = item_history.clone();
                bla.pop();
                for h in bla {
                    *memory.entry((h.0, h.1)).or_insert(0) += memory_new_spring;
                }
                continue
            }

            if !item_spring.contains('?') || fill_count == total {
                let add_count: usize = match arrangement_is_valid(&item_spring, &item_legend) {
                    true => 1,
                    _ => 0
                };
                
                number_of_arrangements += add_count;
                for h in item_history {
                    *memory.entry((h.0, h.1)).or_insert(0) += add_count;
                }
                continue
            }

            let new_spring_and_legend: (String, Vec<usize>) = simplify_spring_and_legend(&item_spring.replacen("?", ".", 1), &(item_legend.clone()));
            if new_spring_and_legend != (EMPTY_STRING, vec![1]) {
                let new_spring: String = new_spring_and_legend.0;
                let new_legend: Vec<usize> = new_spring_and_legend.1;

                let mut new_item_history: Vec<(String, Vec<usize>)> = item_history.clone();
                new_item_history.push((new_spring.clone(), new_legend.clone()));

                queue.push((new_spring.clone(), fill_count, new_legend, new_item_history.clone()));
            }

            let new_spring_and_legend: (String, Vec<usize>) = simplify_spring_and_legend(&item_spring.replacen("?", "#", 1), &item_legend);
            if new_spring_and_legend != (EMPTY_STRING, vec![1]) {
                let new_spring: String = new_spring_and_legend.0;
                let new_legend: Vec<usize> = new_spring_and_legend.1;
                let new_fill_count = new_spring.chars().filter(|&c| c == '#').count();
                if new_fill_count > total {
                    continue
                }

                let mut new_item_history: Vec<(String, Vec<usize>)> = item_history.clone();
                new_item_history.push((new_spring.clone(), new_legend.clone()));

                queue.push((new_spring.clone(), fill_count, new_legend.clone(), new_item_history.clone()));
            }
        }
    }

    // println!("{:?}", memory);

    number_of_arrangements
}


fn get_number_of_arrangements_parallel(springs: &Vec<(&str, Vec<usize>)>) -> usize {
    springs.par_iter().map(|spring| {
        let mut number_of_arrangements: usize = 0;
        let mut memory: HashMap<(String, Vec<usize>), usize> = HashMap::new();
        println!("{:?}", spring);
        let legend: Vec<usize> = spring
            .1
            .iter()
            .cloned()
            .cycle()
            .take((REPEAT_TIMES) * spring.1.len())
            .collect();

        let total: usize = legend.iter().sum();
        
        let mut queue: Vec<(String, usize, Vec<usize>, Vec<(String, Vec<usize>)>)> = vec![
            (
                vec![spring.0.to_string(); REPEAT_TIMES].join("?"),
                0,
                legend.clone(),
                Vec::new()
            )
        ];

        // println!("{:?}", simplify_spring(&vec![spring.0.to_string(); REPEAT_TIMES].join("?"), &legend));

        queue[0].1 = queue[0].0.chars().filter(|&c| c == '#').count();
        queue[0].3.push((simplify_spring(&vec![spring.0.to_string(); REPEAT_TIMES].join("?"), &legend), legend.clone()));

        while !queue.is_empty() {
            let item: (String, usize, Vec<usize>, Vec<(String, Vec<usize>)>) = queue.pop().unwrap();
            let item_spring: String = item.0;
            let fill_count: usize = item.1;
            let item_legend: Vec<usize> = item.2;
            let item_history: Vec<(String, Vec<usize>)> = item.3;

            if memory.contains_key(&(item_spring.clone(), item_legend.clone())) {
                let memory_new_spring: usize = *memory.get(&(item_spring.clone(), item_legend.clone())).unwrap();
                number_of_arrangements += memory_new_spring;
                let mut bla = item_history.clone();
                bla.pop();
                for h in bla {
                    *memory.entry((h.0, h.1)).or_insert(0) += memory_new_spring;
                }
                continue
            }

            if !item_spring.contains('?') || fill_count == total {
                let add_count: usize = match arrangement_is_valid(&item_spring, &item_legend) {
                    true => 1,
                    _ => 0
                };
                
                number_of_arrangements += add_count;
                for h in item_history {
                    *memory.entry((h.0, h.1)).or_insert(0) += add_count;
                }
                continue
            }

            let new_spring_and_legend: (String, Vec<usize>) = simplify_spring_and_legend(&item_spring.replacen("?", ".", 1), &(item_legend.clone()));
            if new_spring_and_legend != (EMPTY_STRING, vec![1]) {
                let new_spring: String = new_spring_and_legend.0;
                let new_legend: Vec<usize> = new_spring_and_legend.1;

                let mut new_item_history: Vec<(String, Vec<usize>)> = item_history.clone();
                new_item_history.push((new_spring.clone(), new_legend.clone()));

                queue.push((new_spring.clone(), fill_count, new_legend, new_item_history.clone()));
            }

            let new_spring_and_legend: (String, Vec<usize>) = simplify_spring_and_legend(&item_spring.replacen("?", "#", 1), &item_legend);
            if new_spring_and_legend != (EMPTY_STRING, vec![1]) {
                let new_spring: String = new_spring_and_legend.0;
                let new_legend: Vec<usize> = new_spring_and_legend.1;
                let new_fill_count = new_spring.chars().filter(|&c| c == '#').count();
                if new_fill_count > total {
                    continue
                }

                let mut new_item_history: Vec<(String, Vec<usize>)> = item_history.clone();
                new_item_history.push((new_spring.clone(), new_legend.clone()));

                queue.push((new_spring.clone(), fill_count, new_legend.clone(), new_item_history.clone()));
            }
        }

        // println!("{:?}", memory);

        number_of_arrangements
    })
    .sum()
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


fn simplify_spring_and_legend(spring: &String, legend: &Vec<usize>) -> (String, Vec<usize>) {
    let mut new_spring: String = spring.clone();
    let mut new_legend: Vec<usize> = legend.clone();

    let mut current_spring_index: usize = 0;

    loop {
        if current_spring_index == spring.len() {
            return ("".to_string(), new_legend);
        }

        match spring.chars().nth(current_spring_index).unwrap() {
            '#' => {
                if new_legend.len() == 0 {
                    return (EMPTY_STRING, vec![1]);
                }

                for _ in 0..new_legend[0] - 1 {
                    current_spring_index += 1;
                    if current_spring_index == spring.len() || spring.chars().nth(current_spring_index).unwrap() == '.' {
                        return (EMPTY_STRING, vec![1]);
                    }
                    new_spring.replace_range(current_spring_index..current_spring_index + 1, "#");
                }

                current_spring_index += 1;
                new_legend.remove(0);

                if current_spring_index == spring.len() {
                    return (new_spring[current_spring_index..].to_string(), new_legend);
                }

                match spring.chars().nth(current_spring_index).unwrap() {
                    '#' => return (EMPTY_STRING, vec![1]),
                    '?' => {
                        new_spring.replace_range(current_spring_index..current_spring_index + 1, ".");
                        return (new_spring[current_spring_index..].to_string(), new_legend);
                    }
                    _ => {}
                }
            },
            '?' => return (new_spring[current_spring_index..].to_string(), new_legend),
            _ => {}

        }

        current_spring_index += 1;
    }
}
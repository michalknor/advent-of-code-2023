use std::fs::File;
use std::io::Read;

use std::cmp;
use std::sync::{Arc, Mutex};
use rayon::prelude::*;

use std::collections::HashMap;

use std::time::Instant;
use colored::Colorize;


const EMPTY_STRING: String = String::new();
const NOT_POSSIBLE: (String, Vec<usize>) = (EMPTY_STRING, vec![1]);


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

    get_number_of_arrangements_parallel2(&springs).to_string()
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


fn get_number_of_arrangements_parallel2(springs: &Vec<(&str, Vec<usize>)>) -> usize {
    let mut min_location = Arc::new(Mutex::new(usize::MIN));

    springs.par_iter().map(|spring| {
        let now = Instant::now();
        let mut number_of_arrangements_parts: [usize; 5] = [0, 0, 0, 0, 0];
        for i in 0..5 {
            if i == 2 || i == 3 {
                continue;
            }
            let legend: Vec<usize> = spring
                .1
                .iter()
                .cloned()
                .cycle()
                .take((i+1) * spring.1.len())
                .collect();

            let total: usize = legend.iter().sum();

            let mut memory: HashMap<(String, Vec<usize>), usize> = HashMap::new();
            
            let mut queue: Vec<(String, usize, Vec<usize>, Vec<String>)> = vec![
                (
                    simplify_spring(&vec![spring.0.to_string(); i+1].join("?"), &legend),
                    0,
                    legend,
                    Vec::new()
                )
            ];

            queue[0].1 = queue[0].0.chars().filter(|&c| c == '#').count();

            while !queue.is_empty() {
                let item: (String, usize, Vec<usize>, Vec<String>) = queue.pop().unwrap();
                let item_spring: String = item.0;
                let fill_count: usize = item.1;
                let item_history: Vec<usize> = item.2;
                let history: Vec<String> = item.3;

                if !item_spring.contains('?') || fill_count == total {
                    if arrangement_is_valid(&item_spring, &legend) {
                        number_of_arrangements_parts[i] += 1;
                        for h in history {
                            *memory.entry((h, item_history)).or_insert(0) += 1;
                        }
                    }
                    continue
                }

                let new_spring_and_legend: (String, Vec<usize>) = simplify_spring_and_legend(&item_spring.replacen("?", ".", 1), &legend);
                if new_spring_and_legend != NOT_POSSIBLE {
                    let new_spring: String = new_spring_and_legend.0;
                    let new_legend: Vec<usize> = new_spring_and_legend.1;
                    let mut new_history: Vec<String> = history.clone();
                    new_history.push(new_spring.clone());
                    queue.push((new_spring, fill_count, new_legend, new_history.clone()));
                }

                let new_spring_and_legend: (String, Vec<usize>) = simplify_spring_and_legend(&item_spring.replacen("?", ".", 1), &legend);
                if new_spring != "" {
                    let new_fill_count = new_spring.chars().filter(|&c| c == '#').count();
                    if new_fill_count > total {
                        continue
                    }

                    let mut new_history: Vec<String> = history.clone();
                    new_history.push(new_spring.clone());

                    if memory.contains_key(&new_spring) {
                        let memory_new_spring: usize = *memory.get(&new_spring).unwrap();
                        number_of_arrangements_parts[i] += memory_new_spring;
                        println!("asd");
                        for h in history {
                            *memory.entry(h).or_insert(0) += memory_new_spring;
                        }
                    }

                    queue.push((new_spring, new_fill_count, new_history.clone()));
                }
            }

            // println!("{:?}", memory);
            // println!("{}", number_of_arrangements_parts[i]);

            if i == 1 && number_of_arrangements_parts[1] % number_of_arrangements_parts[0] == 0 {
                let number_of_arrangements: usize = cmp::max(
                    number_of_arrangements_parts[0] * (number_of_arrangements_parts[1] / number_of_arrangements_parts[0]).pow(4), 
                    number_of_arrangements_parts[0]);
                let mut min_location_guard = min_location.lock().unwrap();
                *min_location_guard = *min_location_guard + 1;
                println!("{:?}: {}, {}, count={}", spring, format!("{:.2?}", now.elapsed()).underline(), number_of_arrangements, min_location_guard);
                return number_of_arrangements;
            }

            // println!("{}", number_of_arrangements[i]);
        }
        let mut min_location_guard = min_location.lock().unwrap();
        *min_location_guard = *min_location_guard + 1;
        println!("{:?}: {}, {}, count={}", spring, format!("{:.2?}", now.elapsed()).underline(), number_of_arrangements_parts[4], min_location_guard);
        number_of_arrangements_parts[4]
    })
    .sum()
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
            return (new_spring, new_legend);
        }

        match spring.chars().nth(current_spring_index).unwrap() {
            '#' => {
                if 0 == legend.len() {
                    return NOT_POSSIBLE;
                }

                for _ in 0..legend[0] - 1 {
                    current_spring_index += 1;
                    if current_spring_index == spring.len() || spring.chars().nth(current_spring_index).unwrap() == '.' {
                        return NOT_POSSIBLE;
                    }
                    new_spring.replace_range(current_spring_index..current_spring_index + 1, "#");
                }

                current_spring_index += 1;
                new_legend.remove(0);

                if current_spring_index == spring.len() {
                    return (new_spring, new_legend);
                }

                match spring.chars().nth(current_spring_index).unwrap() {
                    '#' => return NOT_POSSIBLE,
                    '?' => {
                        new_spring.replace_range(current_spring_index..current_spring_index + 1, ".");
                        return (new_spring, new_legend);
                    }
                    _ => {}
                }
            },
            '?' => return (new_spring, new_legend),
            _ => {}

        }

        current_spring_index += 1;
    }
} 


//("???#####.??????????", [7, 5]): 12.69s, 17576, count=1


//input1 - 13089004412   ???????#.?????????? 7,5 bottleneck
//input2-5 - 4750582079394 but without this row -> .??#??#???????????? 12,1 
//4763671083806 - low
//4763671870238 - low
//4763675870238 - low
//268 needs to be optimized



//129732496415 first 200


// ("????.??.????##??.???", [3, 1, 6, 1, 1]): 320.31ms, 175895226
// ("#?.##?#?#.?.?##", [1, 6, 1, 2]): 206.70µs, 1
// ("?##??#??.#?.#", [2, 1, 1, 1]): 146.00µs, 1
// ("???????#?.?.??", [8, 2]): 1.20ms, 162
// ("#?????#??.", [1, 6]): 161.50µs, 32
// ("?#??#?.????", [5, 1]): 2.23ms, 52488
// ("##?????.????.", [4, 1, 1]): 6.41ms, 5845851
// ("???????##???.?#?.#", [6, 2, 2, 1, 1]): 1.26ms, 1
// ("?#?????#?????", [3, 5]): 9.29ms, 145152
// ("????????##", [1, 1, 3]): 11.78ms, 1303210
// ("????.?????#?", [4, 2, 3]): 595.20µs, 768
// ("??.????????.??###..", [1, 3, 3, 4]): 26.76ms, 39366
// ("?????????#?.#", [1, 3, 4, 1]): 1.03ms, 1024
// (".##??????#????", [5, 6]): 274.90µs, 162
// ("????#??#??.??#?.?#.#", [8, 1, 1, 1]): 1.94ms, 243
// ("?#??????#???????", [2, 4, 1, 2]): 79.03ms, 102515625
// ("#??.?#????.???", [1, 1, 5, 1]): 1.83ms, 24576
// ("?????..???", [3, 1]): 2.30ms, 1600000
// ("?#?.?##.????.?..#???", [2, 3, 4, 1, 1, 1]): 4.66ms, 5184
// ("???.??????????", [1, 10]): 1.08ms, 3888
// ("?##....?#?..", [2, 3]): 34.90µs, 1
// ("##?????#?.????#?.?.", [3, 3, 5, 1]): 2.73ms, 16384
// ("##??.??.#..???.", [4, 1, 1, 1, 1]): 782.30µs, 512
// ("#???.#?????.", [1, 1, 5]): 353.90µs, 32
// ("?????#?#?#?#", [2, 5]): 465.50µs, 1
// ("?#?#?.?????", [3, 1]): 584.50µs, 6480
// ("????#?#.?.", [3, 1]): 210.60µs, 1250
// ("?????.??#??????", [1, 1, 1, 5]): 111.15ms, 295526720
// ("???#?##???#?#.?", [11, 1]): 300.70µs, 162
// ("?#?#?#..??#??.?#??..", [6, 1, 3, 3]): 906.50µs, 32
// day12 part2: 582984534


/*
("?##....?#?..", [2, 3]): 1.48ms, 1, count=1
("#?.##?#?#.?.?##", [1, 6, 1, 2]): 78.34ms, 1, count=2
("?##??#??.#?.#", [2, 1, 1, 1]): 56.55ms, 1, count=3
("?????#?#?#?#", [2, 5]): 60.63ms, 1, count=4
(".##??????#????", [5, 6]): 161.56ms, 162, count=5
("???#?##???#?#.?", [11, 1]): 239.28ms, 232, count=6
("?#?#?#..??#??.?#??..", [6, 1, 3, 3]): 273.57ms, 32, count=7
("????#?#.?.", [3, 1]): 456.99ms, 1784, count=8
("#???.#?????.", [1, 1, 5]): 184.31ms, 32, count=9
("##??.??.#..???.", [4, 1, 1, 1, 1]): 1.04s, 512, count=10
("#?????#??.", [1, 6]): 13.07ms, 32, count=11
("????#??#??.??#?.?#.#", [8, 1, 1, 1]): 1.21s, 243, count=12
("???????#?.?.??", [8, 2]): 1.42s, 162, count=13
("????????##", [1, 1, 3]): 165.02s, 1405000, count=23
("???.??????????", [1, 10]): 193.23s, 8187, count=24
("?????..???", [3, 1]): 234.29s, 2717148, count=25
("##?????.????.", [4, 1, 1]): 322.83s, 5845851, count=26
*/
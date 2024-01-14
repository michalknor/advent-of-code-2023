use std::fs::File;
use std::io::Read;

use std::collections::HashMap;
use queues::*;


const FINAL_NODE: &str = "rx";


#[derive(Debug, PartialEq, Clone)]
enum ModuleType {
    FlipFlop,
    Conjunction,
    Broadcaster
}


#[derive(Debug, PartialEq, Clone)]
enum PulseType {
    Low,
    High
}


#[derive(Debug, Clone)]
pub struct Module {
    module_type: ModuleType,
    send_to: Vec<String>,
}


pub fn main(filename: &str) -> String {
    let mut file = File::open(filename).expect("Failed to open file");
	let mut file_content: String = String::new();

	file.read_to_string(&mut file_content).expect("Failed to read file content");

    let modules: HashMap<String, Module> = file_content
        .lines()
        .map(|line| {
                let (left_side, right_side) = line.split_once(" -> ").unwrap();

                let (module_type, name) = match left_side.chars().next().unwrap() {
                    '%' => (ModuleType::FlipFlop, left_side[1..].to_string()),
                    '&' => (ModuleType::Conjunction, left_side[1..].to_string()),
                    _ => (ModuleType::Broadcaster, left_side.to_string())
                };

                (name, Module { 
                    module_type: module_type,
                    send_to: right_side.split(", ").map(String::from).collect()
                    }
                )
            }
        )
        .collect();

    solve(&modules).to_string()
}


fn solve(modules: &HashMap<String, Module>) -> usize {
    let mut conjunction_map: HashMap<String, HashMap<String, PulseType>> = get_conjunction_map(&modules);

    let mut flip_flop_map: HashMap<String, bool> = modules
        .iter()
        .filter(|(_, module)| module.module_type == ModuleType::FlipFlop)
        .map(|module| (module.0.to_string(), false))
        .collect();

    let last_nodes: Vec<String> = modules
        .iter()
        .filter(|(_, module)| module.send_to.contains(&FINAL_NODE.to_string()))
        .map(|module| module.0.to_string())
        .collect();

    let mut nodes_before_last_nodes: HashMap<String, usize> = modules
        .iter()
        .filter(|(_, module)| {
                for send_to in &module.send_to {
                    if last_nodes.contains(send_to) {
                        return true;
                    }
                }
                false
            }
        )
        .map(|module| (module.0.to_string(), 0))
        .collect();

    let mut queue = Queue::new();

    let mut i: usize = 0;

    'outer: loop {
        i += 1;

        if let Some(send_to) = modules.get("broadcaster").and_then(|module| Some(module.send_to.clone())) {
            for next_module_name in send_to.iter() {
                let _ = queue.add((
                    String::from("broadcast"),
                    next_module_name.clone(),
                    PulseType::Low,
                ));
            }
        }

        while queue.size() > 0 {
            let (origin, current, mut pulse) = queue.remove().unwrap();

            if current == "rx" && pulse == PulseType::Low {
                if i % 100_000 == 0 {
                    println!("{i}:rx = {}", pulse == PulseType::Low);
                }
            }

            if nodes_before_last_nodes.contains_key(&current) && pulse == PulseType::Low {
                if *(nodes_before_last_nodes.get(&current).unwrap()) == 0 {
                    nodes_before_last_nodes.insert(current.clone(), i);
                }

                if nodes_before_last_nodes.values().all(|current| *current != 0) {
                    break 'outer;
                }
            }

            if !modules.contains_key(&current) {
                continue;
            }

            let module: &Module = modules.get(&current).unwrap();

            match module.module_type {
                ModuleType::FlipFlop => {
                    if pulse == PulseType::High {
                        continue;
                    }

                    if let Some(status) = flip_flop_map.get_mut(&current) {
                        *status = !*status;
                        pulse = match status {
                            true => PulseType::High,
                            _ => PulseType::Low
                        };
                    }
                },
                ModuleType::Conjunction => {
                    conjunction_map.get_mut(&current).unwrap().insert(origin.clone(), pulse);
                    pulse = match conjunction_map.get(&current).unwrap().values().all(|current| *current == PulseType::High) {
                        true => PulseType::Low,
                        _ => PulseType::High
                    }
                },
                _ => {}
            }

            for send_to in &module.send_to {
                let _ = queue.add((current.clone(), send_to.clone(), pulse.clone()));
            }

        }
    }

    least_common_multiple(&nodes_before_last_nodes
        .values()
        .cloned()
        .collect::<Vec<usize>>()
    )
}


fn get_conjunction_map(modules: &HashMap<String, Module>) -> HashMap<String, HashMap<String, PulseType>> {
    let mut conjunction_map: HashMap<String, HashMap<String, PulseType>> = HashMap::new();

    for module in modules {
        for module_name in &module.1.send_to {
            if !modules.contains_key(module_name) {
                continue;
            }
            if modules.get(module_name).unwrap().module_type == ModuleType::Conjunction {
                conjunction_map.entry(module_name.to_string()).or_insert(HashMap::new()).insert(module.0.to_string(), PulseType::Low);
            }
        }
    }

    conjunction_map
}


fn least_common_multiple(number_of_steps: &Vec<usize>) -> usize {
    number_of_steps
		.iter()
        .fold(
			number_of_steps[0], 
			|result, &number| 
			result * number / greatest_common_divisor(number, result)
		)
}


fn greatest_common_divisor(smaller_number: usize, larger_number: usize) -> usize {
	(1..=smaller_number)
		.rev()
		.find(|&i| 
			smaller_number % i == 0 && larger_number % i == 0
		)
		.unwrap_or(1)
}
use std::fs::File;
use std::io::Read;

use std::collections::HashMap;


const NODE_ACCEPT: char = 'A';
const NODE_REJECT: char = 'R';


#[derive(Debug)]
pub struct Rule {
    start: u16,
    end: u16,
    category: String,
    send_to: String,
}


pub fn main(filename: &str) -> String {
    let mut file = File::open(filename).expect("Failed to open file");
	let mut file_content: String = String::new();

	file.read_to_string(&mut file_content).expect("Failed to read file content");

    let (first_part, second_part) = file_content
        .split_once("\r\n\r\n")
        .unwrap();

    let workflow: HashMap<String, Vec<Rule>> = first_part
        .lines()
        .map(|line| {
            let (key, rules) = line.split_once("{").unwrap();
            let rules: Vec<Rule> = rules
                .replace("}", "")
                .split(",")
                .map(|unparsed_rule| {
                        if !unparsed_rule.contains(":") {
                            return Rule{start: u16::MIN, end: u16::MAX, category: String::from("*"), send_to: unparsed_rule.to_string()}
                        }
                        let category: String = unparsed_rule
                            .chars()
                            .next()
                            .unwrap()
                            .to_string();
                        let send_to: String = unparsed_rule
                            .find(':')
                            .map(|length| &unparsed_rule[length + 1..])
                            .unwrap()
                            .to_string();
                        if unparsed_rule.contains('<') {
                            let end: u16 = unparsed_rule
                                .find('<')
                                .and_then(|lt| unparsed_rule
                                    .find(':')
                                    .map(|colon| &unparsed_rule[lt + 1..colon])
                                )
                                .unwrap()
                                .parse()
                                .unwrap();
                            return Rule { start: u16::MIN, end: end, category: category, send_to: send_to}
                        }
                        let start: u16 = unparsed_rule
                            .find('>')
                            .and_then(|lt| unparsed_rule
                                .find(':')
                                .map(|colon| &unparsed_rule[lt + 1..colon])
                            )
                            .unwrap()
                            .parse()
                            .unwrap();
                        return Rule { start: start, end: u16::MAX, category: category, send_to: send_to}
                        
                    }
                )
                .collect();
            (key.to_string(), rules)
        })
        .collect();

    let parts: Vec<HashMap<String, u16>> = second_part
        .lines()
        .map(|line| {
            line
                .replace("{", "")
                .replace("}", "")
                .split(",")
                .map(|unparsed_part| {
                        let (category, value) = unparsed_part.split_once("=").unwrap();
                        (category.to_owned(), value.parse().unwrap())
                    }
                )
                .collect()
            }
        )
        .collect();

    get_sum_of_all_accepted_parts(&workflow, &parts).to_string()
}


fn get_sum_of_all_accepted_parts(workflow: &HashMap<String, Vec<Rule>>, parts: &Vec<HashMap<String, u16>>) -> usize {
    let node_accept: String = NODE_ACCEPT.to_string();
    let node_reject: String = NODE_REJECT.to_string();

    let mut stack: Vec<(String, HashMap<String, (u16, u16)>)> = vec![
        (
            String::from("in"),
            HashMap::from(
                [
                    (String::from("x"), (1, 4000)),
                    (String::from("m"), (1, 4000)),
                    (String::from("a"), (1, 4000)),
                    (String::from("s"), (1, 4000))
                ]
            )
        )
    ];

    let mut sum: usize = 0;

    while !stack.is_empty() {
        let item: (String, HashMap<String, (u16, u16)>) = stack.pop().unwrap();

        if item.0 == NODE_ACCEPT.to_string() {
            sum += item.1
                .values()
                .fold::<usize>(
                    0,
                    |result, interval: &(u16, u16)|
                    let switch
                );
            continue;
        }

        if item.0 == NODE_REJECT.to_string() {
            continue;
        }

        let 

        for rule in workflow.get(&item.0).unwrap() {
            if rule.category == "*" {
                stack.push();
            }
        }
    }
    
    sum
}


fn get_next_node(rules: &Vec<Rule>, part: &HashMap<String, u16>) -> String {
    for rule in rules {
        if rule.category == "*" {
            return rule.send_to.to_string();
        }
        if (rule.start..rule.end).contains(part.get(&rule.category).unwrap()) {
            return rule.send_to.to_string();
        }
    }
    NODE_REJECT.to_string()
}
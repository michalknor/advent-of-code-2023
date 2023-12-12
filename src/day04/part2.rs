use std::collections::HashMap;

fn update_tickets(tickets: &mut HashMap<u32, u32>, from_ticket: u32, to_ticket: u32) {
    for ticket_id in from_ticket..to_ticket {
		*tickets.entry(ticket_id).or_insert(0) += *tickets.get(&(from_ticket-1)).unwrap_or(&0);
    }
}

fn evaluate_ticket(winning_numbers: Vec<u32>, your_numbers: Vec<u32>) -> u32 {
	let mut result = 0;

	for winning_number in winning_numbers {
		if your_numbers.contains(&winning_number) {
            result += 1;
        }
	}

	result
}


pub fn main(testing: bool) -> String {
	let file_content: &str = if testing {
		include_str!("test.txt")
	}
	else {
		include_str!("input.txt")
	};

	let mut tickets: HashMap<u32, u32> = HashMap::new();
	let mut ticket_id: u32 = 1;

	for line in file_content.lines() {
		*tickets.entry(ticket_id).or_insert(0) += 1;

		let modified_line: Vec<String> = line.replace("  ", " ").split(": ").nth(1).unwrap_or("").split(" | ").map(String::from).collect();
		
		let winning_numbers: Vec<u32> = modified_line[0].split(' ').map(|s| s.parse::<u32>().expect("Failed to parse as u32")).collect();
		let your_numbers: Vec<u32> = modified_line[1].split(' ').map(|s| s.parse::<u32>().expect("Failed to parse as u32")).collect();

		let result: u32 = evaluate_ticket(winning_numbers, your_numbers);

		update_tickets(&mut tickets, ticket_id + 1, ticket_id + result + 1);

		ticket_id += 1;
    }

	let result: u32 = tickets.values().sum();

    result.to_string()
}
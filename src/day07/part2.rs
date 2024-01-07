use std::fs::File;
use std::io::Read;
use std::collections::HashMap;


#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum PokerHand {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}


fn find_strongest_card(hand: &str) -> char {
	let mut char_count: HashMap<char, u8> = HashMap::new();

	for c in hand.chars() {
		if c == '0' {
			*char_count.entry(c).or_insert(0) = 0;
			continue;
		}
		*char_count.entry(c).or_insert(0) += 1;
	}

	char_count
		.into_iter()
		.max_by_key(|&(_, count)| count)
		.map(|(character, _)| character)
		.unwrap_or_default()
}


fn get_hand_strength(hand: &str) -> PokerHand {
	let hand_tmp = hand;
	let hand2 = &hand_tmp.replace('0', find_strongest_card(hand_tmp).to_string().as_str());
    let mut label_counts: Vec<usize> = hand2
		    .chars()
		    .collect::<Vec<char>>()
		    .iter()
		    .map(|&label| hand2
			    .chars()
			    .filter(|&c| c == label)
			    .count())
		    .filter(|&c| c != 1)
		    .collect();

    label_counts.sort();
    label_counts.reverse();

    match label_counts.as_slice() {
        [5, 5, 5, 5, 5] => PokerHand::FiveOfAKind,
        [4, 4, 4, 4] => PokerHand::FourOfAKind,
        [3, 3, 3, 2, 2] => PokerHand::FullHouse,
        [3, 3, 3] => PokerHand::ThreeOfAKind,
        [2, 2, 2, 2] => PokerHand::TwoPair,
        [2, 2] => PokerHand::OnePair,
        _ => PokerHand::HighCard
    }
}


fn compare_hands(hand_1: &str, hand_2: &str) -> std::cmp::Ordering {
	let hand_strength_1 = get_hand_strength(hand_1);
	let hand_strength_2 = get_hand_strength(hand_2);

	match &hand_strength_1.cmp(&hand_strength_2) {
        std::cmp::Ordering::Greater => std::cmp::Ordering::Greater,
        std::cmp::Ordering::Less => std::cmp::Ordering::Less,
        std::cmp::Ordering::Equal => hand_2.cmp(hand_1),
    }
}

pub fn main(filename: &str) -> String {
    let mut file: File = File::open(filename).expect("Failed to open file");
	let mut file_content: String = String::new();


	file.read_to_string(&mut file_content).expect("Failed to read file content");

	let file_content = &(file_content
		.replace('A', "E")
		.replace('K', "D")
		.replace('Q', "C")
		.replace('J', "0")
		.replace('T', "A"));
	
    let mut hands: Vec<[&str; 2]> = file_content
		.split_whitespace()
		.collect::<Vec<&str>>()
		.chunks_exact(2)
		.map(|chunk| [chunk[0], chunk[1]])
		.collect();

	hands.sort_by(|hand_1, hand_2| compare_hands(hand_2[0], hand_1[0]));

	let result: u64 = hands.iter()
		.enumerate()
		.map(|(index, pair)| {
			let x = pair[1].parse::<u64>().unwrap();
			let y = (index + 1) as u64;
			x * y
		})
		.sum();

	result.to_string()
}
use std::collections::HashMap;

mod day01;
mod day02;
mod day03;
mod day04;

use std::time::Instant;
use std::env;

use colored::Colorize;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    let mut function_map: HashMap<&str, fn(bool)> = HashMap::new();
    
    function_map.insert("day01 part1", day01::part1::main as fn(bool));
    function_map.insert("day01 part2", day01::part2::main as fn(bool));

    function_map.insert("day02 part1", day02::part1::main as fn(bool));
    function_map.insert("day02 part2", day02::part2::main as fn(bool));

    function_map.insert("day03 part1", day03::part1::main as fn(bool));
    function_map.insert("day03 part2", day03::part2::main as fn(bool));

    function_map.insert("day04 part1", day04::part1::main as fn(bool));
    function_map.insert("day04 part2", day04::part2::main as fn(bool));

    if args.len() == 1 {
        let mut sorted_keys: Vec<&&str> = function_map.keys().collect();
        sorted_keys.sort();

        for key in sorted_keys {
            let now = Instant::now();

            print!("{}: ", key);
            function_map[key](false);

            println!("{}: {}", "Elapsed".green().bold(), format!("{:.2?}", now.elapsed()).underline());
        }

        return Ok(());
    }
    
    if args.len() < 3 {
        panic!("ERROR: 2 arguments required")
    }
    
    let call_function: &str = &format!("{} {}", args[1], args[2]);

    if let Some(&func) = function_map.get(&call_function) {
        let now = Instant::now();

        print!("{}: ", call_function);
        func(args.len() > 3);

        println!("{}: {}", "Elapsed".green().bold(), format!("{:.2?}", now.elapsed()).underline());
    }
    
    Ok(())
}


// fn main() {
//     println!(
//         "{}, {}, {}, {}, {}, {}, and some normal text.",
//         "Bold".bold(),
//         "Red".red(),
//         "Yellow".yellow(),
//         "Green Strikethrough".green().strikethrough(),
//         "Blue Underline".blue().underline(),
//         "Purple Italics".purple().italic()
//     );
// }
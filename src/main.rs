use std::collections::HashMap;

mod day01;
mod day02;
mod day03;
mod day04;

use std::fmt::Error;
use std::time::Instant;
use std::env;

fn main() -> std::io::Result<()> {
    let now = Instant::now();

    let args: Vec<String> = env::args().collect();

    // Check if at least two arguments are provided
    if args.len() < 2 {
        println!("Usage: {} <function>", args[0]);
        Err("2 arguments required");
    }

    // Create a HashMap to map function names to functions
    let mut function_map = HashMap::new();
    
    function_map.insert("--day01 --part1", day01::part1::main as fn(bool));
    function_map.insert("--day01 --part2", day01::part2::main as fn(bool));
    function_map.insert("--day02 --part1", day02::part1::main as fn(bool));
    function_map.insert("--day02 --part2", day02::part2::main as fn(bool));
    function_map.insert("--day03 --part1", day03::part1::main as fn(bool));
    function_map.insert("--day03 --part2", day03::part2::main as fn(bool));
    function_map.insert("--day04 --part1", day04::part1::main as fn(bool));
    function_map.insert("--day04 --part2", day04::part2::main as fn(bool));

    // Match the argument to get the corresponding function
    if let Some(&func) = function_map.get(args[1].as_str()) {
        // Pass "123" as the parameter to the function
        func("123");
    } else {
        println!("Unknown function: {}", args[1]);
    }
    
        let _ = day01::part1::main();
        let _ = day01::part2::main();
        let _ = day02::part1::main();
        let _ = day02::part2::main();
        let _ = day03::part1::main();
        let _ = day03::part2::main();
        let _ = day04::part1::main();
        
    day04::part2::main();

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);

    Ok(())
}
use std::collections::HashMap;
use std::path::Path;

use std::time::Instant;
use std::env;

use colored::Colorize;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod day25;

mod tests;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    let mut function_map: HashMap<&str, fn(&str) -> String> = HashMap::new();
    
    function_map.insert("day01 part1", day01::part1::main);
    function_map.insert("day01 part2", day01::part2::main);

    function_map.insert("day02 part1", day02::part1::main);
    function_map.insert("day02 part2", day02::part2::main);

    function_map.insert("day03 part1", day03::part1::main);
    function_map.insert("day03 part2", day03::part2::main);

    function_map.insert("day04 part1", day04::part1::main);
    function_map.insert("day04 part2", day04::part2::main); 

    function_map.insert("day05 part1", day05::part1::main);
    function_map.insert("day05 part2", day05::part2::main);

    function_map.insert("day06 part1", day06::part1::main);
    function_map.insert("day06 part2", day06::part2::main);

    function_map.insert("day07 part1", day07::part1::main);
    function_map.insert("day07 part2", day07::part2::main);

    function_map.insert("day08 part1", day08::part1::main);
    function_map.insert("day08 part2", day08::part2::main);

    function_map.insert("day09 part1", day09::part1::main);
    function_map.insert("day09 part2", day09::part2::main);

    function_map.insert("day10 part1", day10::part1::main);
    function_map.insert("day10 part2", day10::part2::main);

    function_map.insert("day11 part1", day11::part1::main);
    function_map.insert("day11 part2", day11::part2::main);

    function_map.insert("day12 part1", day12::part1::main);
    function_map.insert("day12 part2", day12::part2::main);

    function_map.insert("day13 part1", day13::part1::main);
    function_map.insert("day13 part2", day13::part2::main);

    function_map.insert("day14 part1", day14::part1::main);
    function_map.insert("day14 part2", day14::part2::main);

    function_map.insert("day15 part1", day15::part1::main);
    function_map.insert("day15 part2", day15::part2::main);

    function_map.insert("day16 part1", day16::part1::main);
    function_map.insert("day16 part2", day16::part2::main);

    function_map.insert("day17 part1", day17::part1::main);
    function_map.insert("day17 part2", day17::part2::main);

    function_map.insert("day18 part1", day18::part1::main);
    function_map.insert("day18 part2", day18::part2::main);

    function_map.insert("day19 part1", day19::part1::main);
    function_map.insert("day19 part2", day19::part2::main);

    function_map.insert("day20 part1", day20::part1::main);
    function_map.insert("day20 part2", day20::part2::main);

    function_map.insert("day21 part1", day21::part1::main);
    function_map.insert("day21 part2", day21::part2::main);

    function_map.insert("day22 part1", day22::part1::main);
    function_map.insert("day22 part2", day22::part2::main);

    function_map.insert("day23 part1", day23::part1::main);
    function_map.insert("day23 part2", day23::part2::main);

    function_map.insert("day24 part1", day24::part1::main);
    function_map.insert("day24 part2", day24::part2::main);

    function_map.insert("day25 part1", day25::part1::main);


    if args.len() == 1 {
        let mut sorted_keys: Vec<&&str> = function_map.keys().collect();
        sorted_keys.sort();
        let now_whole = Instant::now();

        for key in sorted_keys {
            let now = Instant::now();

            println!("{key}: {}", 
                function_map[key](
                    &(
                        Path::new("src")
                            .join(
                                key.split(" ").next().unwrap()
                            )
                            .join("input.txt")
                            .into_os_string()
                            .into_string()
                            .unwrap()
                    )
                )
            );

            println!("{}: {}", "Elapsed".green().bold(), format!("{:.2?}", now.elapsed()).underline());
        }

        println!("{}: {}", "Elapsed - total".green().bold(), format!("{:.2?}", now_whole.elapsed()).underline());

        return Ok(());
    }
    
    if args.len() < 3 {
        panic!("ERROR: 2 arguments required")
    }
    
    let call_function: &str = &format!("{} {}", args[1], args[2]);

    if let Some(&func) = function_map.get(&call_function) {
        let now = Instant::now();

        println!("{call_function}: {}", 
            func(
                &(Path::new("src")
                    .join(&args[1])
                    .join(if args.len() < 4 {"input.txt"} else {&args[3]})
                    .into_os_string()
                    .into_string()
                    .unwrap()
                )
            )
        );

        println!("{}: {}", "Elapsed".green().bold(), format!("{:.2?}", now.elapsed()).underline());
    }
    
    Ok(())
}
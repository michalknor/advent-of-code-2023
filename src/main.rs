use std::collections::HashMap;
use std::path::Path;

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

use std::time::Instant;
use std::env;

use colored::Colorize;

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


    if args.len() == 1 {
        let mut sorted_keys: Vec<&&str> = function_map.keys().collect();
        sorted_keys.sort();

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

#[cfg(test)]
mod tests {
    use std::path::Path;
    
    use crate::day10;
    use crate::day11;
    use crate::day12;

    #[test]
    fn day10_part1() {
        assert_eq!(day10::part1::main(
            &Path::new("src")
                .join("day10")
                .join("test1.txt")
                .into_os_string()
                .into_string()
                .unwrap()),
            "4"
        );
        assert_eq!(day10::part1::main(
            &Path::new("src")
                .join("day10")
                .join("test2.txt")
                .into_os_string()
                .into_string()
                .unwrap()),
            "8"
        );
    }

    #[test]
    fn day10_part2() {
        assert_eq!(day10::part2::main(
            &Path::new("src")
                .join("day10")
                .join("test1.txt")
                .into_os_string()
                .into_string()
                .unwrap()),
            "1"
        );
        assert_eq!(day10::part2::main(
            &Path::new("src")
                .join("day10")
                .join("test2.txt")
                .into_os_string()
                .into_string()
                .unwrap()),
            "1"
        );
        assert_eq!(day10::part2::main(
            &Path::new("src")
                .join("day10")
                .join("test3.txt")
                .into_os_string()
                .into_string()
                .unwrap()),
            "4"
        );
        assert_eq!(day10::part2::main(
            &Path::new("src")
                .join("day10")
                .join("test4.txt")
                .into_os_string()
                .into_string()
                .unwrap()),
            "8"
        );
        assert_eq!(day10::part2::main(
            &Path::new("src")
                .join("day10")
                .join("test5.txt")
                .into_os_string()
                .into_string()
                .unwrap()),
            "10"
        );
    }

    #[test]
    fn day11_part1() {
        assert_eq!(day11::part1::main(
            &Path::new("src")
                .join("day11")
                .join("test1.txt")
                .into_os_string()
                .into_string()
                .unwrap()),
            "374"
        );
    }

    #[test]
    fn day12_part1() {
        assert_eq!(day12::part1::main(
            &Path::new("src")
                .join("day12")
                .join("test1.txt")
                .into_os_string()
                .into_string()
                .unwrap()),
            "21"
        );
    }

    #[test]
    fn day12_part2() {
        assert_eq!(day12::part2::main(
            &Path::new("src")
                .join("day12")
                .join("test1.txt")
                .into_os_string()
                .into_string()
                .unwrap()),
            "525152"
        );
    }
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
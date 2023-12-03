mod day01;
mod day02;
mod day03;

fn main() -> std::io::Result<()> {
    if false {
        let _ = day01::part1::main();
        let _ = day01::part2::main();
        let _ = day02::part1::main();
        let _ = day02::part2::main();
        let _ = day03::part1::main();
    }
    day03::part2::main()
}
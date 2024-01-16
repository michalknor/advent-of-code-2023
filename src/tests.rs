#[cfg(test)]
mod tests {
    use std::path::Path;
    
    use crate::day10;
    use crate::day11;
    use crate::day12;
    use crate::day13;
    use crate::day14;
    use crate::day15;
    use crate::day16;
    use crate::day17;
    use crate::day18;
    use crate::day19;
    use crate::day20;
    use crate::day21;
    use crate::day22;
    use crate::day23;
    use crate::day24;
    use crate::day25;

    fn get_path(day: &str, file: &str) -> String {
        Path::new("src")
                .join(day)
                .join(file)
                .into_os_string()
                .into_string()
                .unwrap()
    }

    #[test]
    fn day10_part1() {
        assert_eq!(day10::part1::main(&get_path("day10", "test1.txt")), "4");
        assert_eq!(day10::part1::main(&get_path("day10", "test2.txt")), "8");
    }

    #[test]
    fn day10_part2() {
        assert_eq!(day10::part2::main(&get_path("day10", "test1.txt")), "1");
        assert_eq!(day10::part2::main(&get_path("day10", "test2.txt")), "1");
        assert_eq!(day10::part2::main(&get_path("day10", "test3.txt")), "4");
        assert_eq!(day10::part2::main(&get_path("day10", "test4.txt")), "8");
        assert_eq!(day10::part2::main(&get_path("day10", "test5.txt")), "10");
    }

    #[test]
    fn day11_part1() {
        assert_eq!(day11::part1::main(&get_path("day11", "test1.txt")), "374");
    }

    #[test]
    fn day12_part1() {
        assert_eq!(day12::part1::main(&get_path("day12", "test1.txt")), "21");
    }

    #[test]
    fn day12_part2() {
        assert_eq!(day12::part2::main(&get_path("day12", "test1.txt")), "525152");
    }

    #[test]
    fn day13_part1() {
        assert_eq!(day13::part1::main(&get_path("day13", "test1.txt")), "405");
    }

    #[test]
    fn day13_part2() {
        assert_eq!(day13::part2::main(&get_path("day13", "test1.txt")), "400");
    }

    #[test]
    fn day14_part1() {
        assert_eq!(day14::part1::main(&get_path("day14", "test1.txt")), "136");
    }

    #[test]
    fn day14_part2() {
        assert_eq!(day14::part2::main(&get_path("day14", "test1.txt")), "64");
    }

    #[test]
    fn day15_part1() {
        assert_eq!(day15::part1::main(&get_path("day15", "test1.txt")), "1320");
    }

    #[test]
    fn day15_part2() {
        assert_eq!(day15::part2::main(&get_path("day15", "test1.txt")), "145");
    }

    #[test]
    fn day16_part1() {
        assert_eq!(day16::part1::main(&get_path("day16", "test1.txt")), "46");
    }

    #[test]
    fn day16_part2() {
        assert_eq!(day16::part2::main(&get_path("day16", "test1.txt")), "51");
    }

    #[test]
    fn day17_part1() {
        assert_eq!(day17::part1::main(&get_path("day17", "test1.txt")), "102");
    }

    #[test]
    fn day17_part2() {
        assert_eq!(day17::part2::main(&get_path("day17", "test1.txt")), "94");
        assert_eq!(day17::part2::main(&get_path("day17", "test2.txt")), "71");
    }
    
    #[test]
    fn day18_part1() {
        assert_eq!(day18::part1::main(&get_path("day18", "test1.txt")), "62");
    }
    
    #[test]
    fn day18_part2() {
        assert_eq!(day18::part2::main(&get_path("day18", "test1.txt")), "952408144115");
    }
    
    #[test]
    fn day19_part1() {
        assert_eq!(day19::part1::main(&get_path("day19", "test1.txt")), "19114");
    }
    
    #[test]
    fn day19_part2() {
        assert_eq!(day19::part2::main(&get_path("day19", "test1.txt")), "167409079868000");
    }
    
    #[test]
    fn day20_part1() {
        assert_eq!(day20::part1::main(&get_path("day20", "test1.txt")), "32000000");
        assert_eq!(day20::part1::main(&get_path("day20", "test2.txt")), "11687500");
    }
    
    #[test]
    fn day22_part1() {
        assert_eq!(day22::part1::main(&get_path("day22", "test1.txt")), "5");
    }
    
    #[test]
    fn day22_part2() {
        assert_eq!(day22::part2::main(&get_path("day22", "test1.txt")), "7");
    }
}
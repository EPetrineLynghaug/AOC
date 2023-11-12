use colored::Colorize;

mod common;
mod part1;
mod part2;

fn main() {
    let input = include_str!("../input.txt");

    println!(
        "{} {}",
        format!("Part 1:").underline(),
        format!("{}", part1::solution(input)).bold()
    );

    println!(
        "{} {}",
        format!("Part 2:").underline(),
        format!("{}", part2::solution(input)).bold()
    );
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT1: &str = ">";
    const INPUT2: &str = "^>v<";
    const INPUT3: &str = "^v^v^v^v^v";

    #[test]
    fn part1_works() {
        assert_eq!(part1::solution(INPUT1), 2);
        assert_eq!(part1::solution(INPUT2), 4);
        assert_eq!(part1::solution(INPUT3), 2);
    }

    const INPUT4: &str = "^v";

    #[test]
    fn part2_works() {
        assert_eq!(part2::solution(INPUT4), 3);
        assert_eq!(part2::solution(INPUT2), 3);
        assert_eq!(part2::solution(INPUT3), 11);
    }
}

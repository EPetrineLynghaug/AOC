use colored::Colorize;

mod part1;
mod part2;

fn main() {
    /*Den faktiske koden*/
    let input = "iwrupvqb";

    /*Part one solution */
    // println!("Part 1: {}", part1::solution(input)).green();
    println!(
        "{} {}",
        format!("Part 1:").underline(),
        format!("{}", part1::solution(input)).bold()
    );

    /*Part two solution */
    println!(
        "{} {}",
        format!("Part 2:").underline(),
        format!("{}", part2::solution(input)).bold()
    );
}
#[cfg(test)] /*cft bare kompiler når. denne kan bare kjøres om du skriver cargo test.  */

mod test {
    use super::*; /*denne  sier at denne er helt unik og ikke har noe med resten av programmet å gjøre. denne må være her for ellers kan den tro du stjeler eller hacker. */

    const INPUT1: &str = "abcdef"; 
    const INPUT2: &str = "pqrstuv";

    #[test]
    fn part1_works() {
        let resulte1 = part1::solution(INPUT1);
        assert_eq!(resulte1, 609043);

        let resulte2 = part1::solution(INPUT2);
        assert_eq!(resulte2, 1048970);
    }
}

   


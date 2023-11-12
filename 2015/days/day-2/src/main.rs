use colored::Colorize;

mod part1;
mod part2;
mod common;


/*PUB er public og kan brukes flere steder. */
fn main() {
    /*Den faktiske koden*/
    let input = include_str!("../input.txt");

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

    const INPUT: &str = "2x3x4"; /*str står for sting. LUFTIG SPRÅK SOM TRENGER LUFT.*/

    #[test]
    fn part1_works() {
        let resulte = part1::solution(INPUT);
        assert_eq!(resulte, 58);
    }

    #[test]
    fn part2_works() {
        let resulte = part2::solution(INPUT);
        assert_eq!(resulte, 34);
    }
}

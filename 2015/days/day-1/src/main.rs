mod part1;
mod part2;

fn main() {
    /*Den faktiske koden*/
    let input = include_str!("../input.txt");

    /*Part one solution */
    println!("Part 1: {}", part1::solution(input));

    /*Part two solution */
    println!("Part 2: {}", part2::solution(input));
}
#[cfg(test)] /*cft bare kompiler når. denne kan bare kjøres om du skriver cargo test.  */

mod test {
    use super::*; /*denne  sier at denne er helt unik og ikke har noe med resten av programmet å gjøre. denne må være her for ellers kan den tro du stjeler eller hacker. */

    const INPUT: &str = "()())"; /*str står for sting. LUFTIG SPRÅK SOM TRENGER LUFT.*/

    #[test]
    fn part1_works() {
        let resulte = part1::solution(INPUT);
        assert_eq!(resulte, -1);
    }

    #[test]
    fn part2_works() {
        let resulte = part2::solution(INPUT);
        assert_eq!(resulte, 5);
    }
}

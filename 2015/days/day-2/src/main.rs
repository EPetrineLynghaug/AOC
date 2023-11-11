use colored::Colorize;
use nom::{
    bytes::complete::tag,
    character::{self, complete::newline},
    multi::separated_list0,
    IResult,
};

mod part1;
mod part2;

/*Lager vår egen data ut i fra listen vi fikk utdelt. kan hete hva den vil men MÅ ha stor bokstav
grunnen til at den er i main, er at par2 og par2  er barna. */
#[derive(Debug)]
pub struct Present {
    length: u32,
    width: u32,
    height: u32,
}

fn parse_present(input: &str) -> IResult<&str, Present> {
    let (input, length) = character::complete::u32(input)?; /*Input str der oppe vil være lik denne. stopper bare om vi finner en u32 */
    let (input, _) = tag("x")(input)?;
    let (input, width) = character::complete::u32(input)?;
    let (input, _) = tag("x")(input)?;
    let (input, height) = character::complete::u32(input)?;
    /*heter å forndøye data  */

    Ok((
        input,
        Present {
            length,
            width,
            height, 
            /*, er god sikk selvom det ikke kommer noe etter. trailing comma */ 
        }
    ))
}
/*_ er at vi ikke vil bruke denne/ lagre den */

/*Iresulte betyr at den kan faile. noe er feil typ. vist alt er rett får jeg et en liten type tekst. */
pub fn parse_presents(input: &str) -> IResult<&str, Vec<Present>> {
    let (input, presents) = separated_list0(newline, parse_present)(input)?;

    Ok((
        input,
        presents
    ))
}
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
        assert_eq!(resulte, todo!());
    }
}

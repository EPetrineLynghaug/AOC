use nom::{
    bytes::complete::tag,
    character::{self, complete::newline},
    multi::separated_list0,
    IResult,
};

/*Lager vår egen data ut i fra listen vi fikk utdelt. kan hete hva den vil men MÅ ha stor bokstav
grunnen til at den er i main, er at par2 og par2  er barna. */
#[derive(Debug)]
pub struct Present {
    pub length: u32,
    pub width: u32,
    pub height: u32,
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
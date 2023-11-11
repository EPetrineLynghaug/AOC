use std::cmp::min;

use crate::parse_presents;

pub fn solution(input: &str) -> u32 {
    let presents = parse_presents(input).expect("Couldn parse the presents").1;
    /*dbg!(presents);*/
    /*tvinger det over til tekst i konsollen og printer det. */

    presents
        .into_iter()
        .map(|presents| {
            let length_side = presents.length * presents.height;
            let width_side = presents.width * presents.height;
            let height_side = presents.length * presents.width;

            let exrea_padding = min(length_side, min(width_side, height_side));

            (length_side * 2) + (width_side * 2) + (height_side * 2) + exrea_padding
            /*når du ikke bruker ; blir det til return */
        })
        .sum::<u32>()
    /*intoiterater into stjeler vaiabelen. du kan ikke lengre bruke den. tatt ownership */
}

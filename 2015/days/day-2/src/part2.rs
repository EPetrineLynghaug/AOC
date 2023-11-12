use crate::common::parse_presents;

pub fn solution(input: &str) -> u32 {
    let presents = parse_presents(input).expect("Couldn parse the presents").1;

    presents
        .into_iter()
        .map(|present| {
            let mut sides = [present.length, present.width, present.height];
            sides.sort();

            sides.get(0).expect("Couldn't get value for array") * 2
                + sides.get(1).expect("Couldn't get value for array") * 2
                + present.length * present.width * present.height
        })
        .sum()
}

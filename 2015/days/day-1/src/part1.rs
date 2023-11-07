pub fn solution(input: &str) -> isize {
    let mut floor: isize = 0; /*mut = muteres. den er 0 men den skal faktisk endres på */

    input.chars().for_each(|char| match char {
        '(' => floor += 1,
        ')' => floor -= 1,
        _=> (),
        /*en string, liste med tegn. .chars gir oss listen. for eatch for hvert tegn i listen. match er en tydelige if else. */
    });
    /* chars en funksjon som tar stringen og gjør det til ne funksjon med alle de induville tegnene */

    floor
}

/* 1Ikke ha med ; betyr at du retuner. gjelder bare på siste linjen*/

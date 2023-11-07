pub fn solution(input: &str) -> usize {
    let mut position: usize = 1;
    let mut floors: isize = 0;

    for char in input.chars() {
        match char {
            '(' => floors += 1,
            ')' => floors -= 1,
            _ => (),
        }


        if floors < 0 {
            return position;
        }


        position += 1;
    }

    position
}

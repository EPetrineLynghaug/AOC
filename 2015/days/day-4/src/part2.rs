use md5::{Digest, Md5};

pub fn solution(input: &str) -> usize {
    let mut i: usize = 0;

    loop {
        i += 1;

        let mut hasher = Md5::new();
        hasher.update((input.to_owned() + &i.to_string()).as_bytes());
        let result =hex::encode(hasher.finalize());


        if result.starts_with("000000") {
            break;
        }
    }

    i
}
    

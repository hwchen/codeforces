use std::io::{self, Read};

fn main() {
    let stdin = io::stdin();
    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();

    let input = buf.trim();
    let input = input.parse().expect("error parsing");

    if can_div_melon(input) {
        println!("YES");
    } else {
        println!("NO");
    }
}

fn can_div_melon(input: u64) -> bool {
    if input == 2 {
        false
    } else {
        input % 2 == 0
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_pave() {
        assert_eq!(true, can_div_melon(8));
        assert_eq!(falsse, can_div_melon(2));
    }
}

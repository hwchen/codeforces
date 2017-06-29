use std::io::{self, BufRead};
use std::iter;

fn main() {
    let stdin = io::stdin();
    let mut stdin = stdin.lock();
    let mut buf = String::new();

    stdin.read_line(&mut buf).unwrap();

    let n: usize = buf.parse().unwrap();

    buf.clear();

    stdin.read_line(&mut buf).unwrap();

    let knight_moods = buf.split_whitespace()
        .map(|x| x.parse())
        .collect::<Result<Vec<_>, _>>()
        .unwrap();
}

fn is_lucky(n: usize, knight_moods: Vec<usize>) {
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_abbreviate() {
        assert_eq!(Bar {
            saturation: 10,
            saturated_n: 5,
            partially_saturated: 4,
            zeroes: 4,
        }, progress(10, 10, 54));
        assert_eq!(Bar {
            saturation: 13,
            saturated_n: 4,
            partially_saturated: 0,
            zeroes: 6,
        }, progress(11, 13, 37));
    }
}

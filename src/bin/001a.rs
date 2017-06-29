use std::io::{self, Read};

fn main() {
    let mut stdin = io::stdin();
    let mut buf = String::new();
    stdin.read_to_string(&mut buf).unwrap();

    let inputs = buf.split_whitespace()
        .map(|d| d.parse())
        .collect::<Result<Vec<_>, _>>()
        .unwrap();

    println!("{}", pave( inputs[0], inputs[1], inputs[2]));
}

fn pave(n: u64, m: u64, a: u64) -> u64 {
    fn edge_pave(edge: u64, pave_len: u64) -> u64 {
        if edge % pave_len == 0 {
            edge / pave_len
        } else {
            edge / pave_len + 1
        }
    }

    let n_pave = edge_pave(n, a);
    let m_pave = edge_pave(m, a);

    n_pave * m_pave
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_pave() {
        assert_eq!(4, pave(6, 6, 4));
        assert_eq!(4, pave(6, 6, 3));
        assert_eq!(4, pave(6, 6, 5));
        assert_eq!(2, pave(6, 2, 5));
        assert_eq!(1, pave(2, 2, 5));
        assert_eq!(1, pave(1, 2, 5));
        assert_eq!(2, pave(1, 2, 1));
        assert_eq!(1, pave(1, 1, 1));
        assert_eq!(1, pave(1, 1, 1));
        assert_eq!(12, pave(7, 10, 3));
        assert_eq!(1, pave(1_000_000_000, 1_000_000_000, 1_000_000_000));
        assert_eq!(1_000_000_000_000_000_000, pave(1_000_000_000, 1_000_000_000, 1));
    }
}

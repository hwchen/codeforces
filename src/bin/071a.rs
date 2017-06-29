use std::io::{self, BufRead, Read};

fn main() {
    let stdin = io::stdin();
    let stdin = stdin.lock();

    let mut lines = stdin.lines();
    let n = lines.next().unwrap().unwrap().parse::<usize>().unwrap();

    for line in lines.take(n) {
        println!("{}", abbreviate(&line.unwrap()));
    }
}

fn abbreviate(input: &str) -> String {
    let cs = input.chars();
    let len = cs.count();

    if len <= 10 {
        input.to_owned()
    } else {
        let mut cs = input.chars();
        let res = format!("{}{}{}",
            cs.next().unwrap(),
            len - 2,
            cs.last().unwrap()
        );
        res

    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_abbreviate() {
        assert_eq!("l10n".to_owned(), abbreviate("localization"));
    }
}


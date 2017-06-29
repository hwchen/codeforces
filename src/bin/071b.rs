use std::io::{self, BufRead, Write, StdoutLock};
use std::iter;

fn main() {
    let stdin = io::stdin();
    let mut stdin = stdin.lock();
    let mut buf = String::new();

    stdin.read_line(&mut buf).unwrap();

    let input = buf.split_whitespace()
        .map(|x| x.parse())
        .collect::<Result<Vec<_>, _>>()
        .unwrap();

    let bar = progress(input[0], input[1], input[2]);

    let stdout = io::stdout();
    let mut stdout = stdout.lock();

    buffered_write(&bar, &mut stdout);
    //simple_write(&bar, &mut stdout);
}

//fn simple_write(bar: &Bar, mut stdout: &mut StdoutLock) {
//    for _ in 0..bar.saturated_n {
//        write!(&mut stdout, "{} ", bar.saturation).unwrap();
//    }
//
//    write!(&mut stdout, "{} ", bar.partially_saturated).unwrap();
//
//    for _ in 0..(bar.zeroes - 1) {
//        write!(&mut stdout, "0 ", ).unwrap();
//    }
//
//    write!(&mut stdout, "0").unwrap();
//
//    stdout.flush().unwrap();
//}

fn buffered_write(bar: &Bar, mut stdout: &mut StdoutLock) {
    // buffered write, 200mb at a time
    let buf_size = 200_000_000;

    // Buffered write of saturated
    for _ in 0..bar.saturated_n / buf_size {
        let res = iter::repeat(format!("{} ", bar.saturation)).take(bar.saturated_n).collect::<String>();
        write!(&mut stdout, "{}", res).unwrap();
        stdout.flush().unwrap();
    }
    let res = iter::repeat(format!("{} ", bar.saturation)).take(bar.saturated_n % buf_size).collect::<String>();
    write!(&mut stdout, "{}", res).unwrap();
    stdout.flush().unwrap();

    // Unbuffered write of middle value
    if let Some(part_sat) = bar.partially_saturated {
        write!(&mut stdout, "{} ", part_sat).unwrap();
    }

    // Buffered write of zeroes
    if bar.zeroes > 0 {
        for _ in 0..(bar.zeroes - 1) / buf_size {
            let res = iter::repeat("0 ").take(buf_size).collect::<String>();
            write!(&mut stdout, "{}", res).unwrap();
            stdout.flush().unwrap();
        }
        let res = iter::repeat("0 ").take((bar.zeroes - 1) % buf_size).collect::<String>();
        write!(&mut stdout, "{}", res).unwrap();
        stdout.flush().unwrap();

        write!(&mut stdout, "0").unwrap();
    }

    stdout.flush().unwrap();
}

#[derive(Debug, PartialEq)]
struct Bar {
    saturation: usize,
    saturated_n: usize,
    partially_saturated: Option<usize>,
    zeroes: usize,
}

fn progress(n: usize, k: usize, t: usize) -> Bar {
    let saturation = (n * k * t) / 100;

    let sat_n = (n * t) /100;
    let part_sat = saturation % k ;

    let unsat_n = if sat_n == n { 0 } else { n - sat_n - 1 };

    let partially_saturated = if sat_n == n { None } else { Some(part_sat) };
    Bar {
        saturation: k,
        saturated_n: sat_n,
        partially_saturated: partially_saturated,
        zeroes: unsat_n,
    }
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



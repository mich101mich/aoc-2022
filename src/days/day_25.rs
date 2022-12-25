use cgmath::conv;

use crate::utils::*;

fn convert(n: &[isize]) -> isize {
    n.iter()
        .rev()
        .enumerate()
        .map(|(i, x)| 5isize.pow(i as u32) * *x)
        .sum::<isize>()
}

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/25.txt");

    let mut parsed = input
        .lines()
        .map(|l| {
            l.bytes()
                .map(|b| match b {
                    b'2' => 2isize,
                    b'1' => 1,
                    b'0' => 0,
                    b'-' => -1,
                    b'=' => -2,
                    _ => panic!(),
                })
                .to_vec()
        })
        .map(|v| convert(&v))
        .sum::<isize>();

    let n = parsed;

    let len = (0..).find(|i| 5isize.pow(*i as u32) > parsed).unwrap();

    let mut digits = vec![0; len + 1];
    let mut max_below = digits
        .iter()
        .enumerate()
        .map(|(i, _)| 2 * 5isize.pow(i as u32))
        .sum::<isize>();

    for (i, x) in digits.iter_mut().enumerate().rev() {
        let pow = 5isize.pow(i as u32);
        max_below -= 2 * pow;
        while parsed > max_below {
            *x += 1;
            parsed -= pow;
        }
        while parsed < -max_below {
            *x -= 1;
            parsed += pow;
        }
    }
    while digits.last() == Some(&0) {
        digits.pop();
    }
    digits.reverse();
    assert_eq!(n, convert(&digits));
    let digits = digits
        .iter()
        .map(|x| match x {
            -2 => '=',
            -1 => '-',
            0 => '0',
            1 => '1',
            2 => '2',
            _ => panic!(),
        })
        .to_vec();
    print_arr!(digits);
}

use crate::utils::*;

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/10.txt");
    let mut x = 1;
    let mut cycle = 0;

    let parsed = input.lines().to_vec();

    let mut screen = vec![false; 40];
    let mut update_screen = |x: isize| {
        let screen_x: usize = cycle % 40;
        if (screen_x as isize - x).abs() < 2 {
            screen[screen_x] = true;
        }
        cycle += 1;
        let screen_x: usize = cycle % 40;
        if screen_x % 40 == 0 {
            print_dotted_line(&screen, '#', ' ');
            screen.fill(false);
        }
    };
    let mut screen_x = 0;

    for l in parsed {
        if let Some(l) = l.strip_prefix("addx ") {
            let n = l.parse::<isize>().unwrap();
            update_screen(x);
            update_screen(x);
            x += n;
        } else {
            update_screen(x);
        }
    }
}

#[allow(unused)]
pub fn part_one() {
    #[allow(unused_variables)]
    let input = include_str!("../input/10.txt");
    let mut x = 1;
    let mut cycle = 0;

    let parsed = input.lines().to_vec();

    let mut sum = 0;

    for l in parsed {
        if let Some(l) = l.strip_prefix("addx ") {
            let n = l.parse::<isize>().unwrap();
            cycle += 1;
            if cycle % 40 == 20 {
                sum += x * cycle;
                pv!(cycle, x, sum);
            }
            cycle += 1;
            if cycle % 40 == 20 {
                sum += x * cycle;
                pv!(cycle, x, sum);
            }
            x += n;
        } else {
            cycle += 1;
            if cycle % 40 == 20 {
                sum += x * cycle;
                pv!(cycle, x, sum);
            }
        }
    }

    pv!(sum);
}

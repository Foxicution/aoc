use aoc::parse::{array, numbers};

#[derive(PartialEq, Debug, Clone, Copy)]
enum Action {
    On,
    Off,
    Toggle,
}

fn parse(input: &str) -> Vec<(Action, [usize; 4])> {
    input
        .trim()
        .lines()
        .map(|l| {
            let bytes = l.as_bytes();
            let action = match bytes[6] {
                b'f' => Action::Off,
                b'n' => Action::On,
                _ => Action::Toggle,
            };
            let nums: [usize; 4] = array(&mut numbers(l));
            (action, nums)
        })
        .collect()
}

fn part1(input: &[(Action, [usize; 4])]) -> usize {
    // 2D tensor repr
    let mut grid = [false; 1000 * 1000];

    for &(action, [x1, y1, x2, y2]) in input {
        grid.chunks_exact_mut(1000)
            .skip(y1)
            .take(y2 - y1 + 1)
            .for_each(|row| match action {
                Action::On => row[x1..=x2].fill(true),
                Action::Off => row[x1..=x2].fill(false),
                Action::Toggle => row[x1..=x2].iter_mut().for_each(|b| *b = !*b),
            });
    }
    grid.iter().filter(|x| **x).count()
}

fn part2(input: &[(Action, [usize; 4])]) -> usize {
    // array causes stack overflow
    let mut grid: Vec<usize> = vec![0; 1000 * 1000];

    for &(action, [x1, y1, x2, y2]) in input {
        grid.chunks_exact_mut(1000)
            .skip(y1)
            .take(y2 - y1 + 1)
            .for_each(|row| match action {
                Action::On => row[x1..=x2].iter_mut().for_each(|i| *i += 1),
                Action::Off => row[x1..=x2]
                    .iter_mut()
                    .for_each(|i| *i = i.saturating_sub(1)),
                Action::Toggle => row[x1..=x2].iter_mut().for_each(|i| *i += 2),
            });
    }
    grid.iter().sum()
}

const INPUT: &str = include_str!("../../../inputs/2015/06.txt");

fn main() {
    let input = parse(INPUT);

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("turn on 0,0 through 999,999", vec![(Action::On, [0, 0, 999, 999])])]
    #[case("toggle 0,0 through 999,0", vec![(Action::Toggle, [0, 0, 999, 0])])]
    #[case("turn off 499,499 through 500,500", vec![(Action::Off, [499, 499, 500, 500])])]
    fn test_parse(#[case] input: &str, #[case] expected: Vec<(Action, [usize; 4])>) {
        assert_eq!(parse(input), expected)
    }

    #[rstest]
    #[case("turn on 0,0 through 999,999", 1000 * 1000)]
    fn test_part1(#[case] input: &str, #[case] expected: usize) {
        assert_eq!(part1(&parse(input)), expected)
    }

    #[rstest]
    #[case("turn on 0,0 through 0,0", 1)]
    #[case("toggle 0,0 through 999,999", 2000000)]
    fn test_part2(#[case] input: &str, #[case] expected: usize) {
        assert_eq!(part2(&parse(input)), expected)
    }
}

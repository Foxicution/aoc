fn parse(input: &str) -> &str {
    input
}

fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let mut parts = line.split('x');
            let (l, w, h): (u32, u32, u32) = (
                parts.next().unwrap().parse().unwrap(),
                parts.next().unwrap().parse().unwrap(),
                parts.next().unwrap().parse().unwrap(),
            );

            let (w1, w2, w3) = (l * w, w * h, h * l);
            let min = w1.min(w2).min(w3);
            2 * (w1 + w2 + w3) + min
        })
        .sum()
}

fn part2(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let mut parts = line.split('x').map(|s| s.parse::<u32>().unwrap());
            let mut dims = [
                parts.next().unwrap(),
                parts.next().unwrap(),
                parts.next().unwrap(),
            ];

            dims.sort();
            let (l, w, h) = (dims[0], dims[1], dims[2]);

            2 * (l + w) + l * w * h
        })
        .sum()
}

const INPUT: &str = include_str!("../../../inputs/2015/02.txt");

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
    #[case("2x3x4", 58)]
    #[case("1x1x10", 43)]
    fn test_part1(#[case] input: &str, #[case] expected: u32) {
        assert_eq!(part1(input), expected)
    }

    #[rstest]
    #[case("2x3x4", 34)]
    #[case("1x1x10", 14)]
    fn test_part2(#[case] input: &str, #[case] expected: u32) {
        assert_eq!(part2(input), expected)
    }
}

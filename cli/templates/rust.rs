fn parse(input: &str) -> &str {
    input.trim()
}

fn part1(input: &str) -> u8 {
    0
}

fn part2(input: &str) -> u8 {
    0
}

const INPUT: &str = include_str!("../../../inputs/{{year}}/{{day}}.txt");

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
    #[case("", 1)]
    fn test_part1(#[case] input: &str, #[case] expected: u8) {
        assert_eq!(part1(input), expected)
    }

    #[rstest]
    #[case("", 1)]
    fn test_part2(#[case] input: &str, #[case] expected: u8) {
        assert_eq!(part2(input), expected)
    }
}

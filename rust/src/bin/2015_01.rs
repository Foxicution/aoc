const INPUT: &str = include_str!("../../../inputs/2015/01.txt");

fn parse(input: &str) -> &str {
    input
}

fn part1(input: &str) -> i32 {
    input
        .trim()
        .bytes()
        .map(|ch| if ch == b'(' { 1 } else { -1 })
        .sum()
}

fn part2(input: &str) -> usize {
    let mut floor = 0;

    for (i, ch) in input.bytes().enumerate() {
        floor += if ch == b'(' { 1 } else { -1 };

        if floor == -1 {
            return i + 1;
        }
    }
    0
}

fn main() {
    let input = parse(INPUT);

    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("(())", 0)]
    #[case("()()", 0)]
    #[case("(((", 3)]
    #[case("(()(()(", 3)]
    #[case("))(((((", 3)]
    #[case("())", -1)]
    #[case("))(", -1)]
    #[case(")))", -3)]
    #[case(")())())", -3)]
    fn test_part1(#[case] input: &str, #[case] expected: i32) {
        assert_eq!(part1(input), expected)
    }

    #[rstest]
    #[case(")", 1)]
    #[case("()())", 5)]
    fn test_part2(#[case] input: &str, #[case] expected: usize) {
        assert_eq!(part2(input), expected)
    }
}

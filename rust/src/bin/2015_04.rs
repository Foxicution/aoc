use aoc::md5;

fn parse(input: &str) -> &str {
    input.trim()
}

fn find_suffix(input: &str, start: u64, check: fn(&[u8]) -> bool) -> u64 {
    for i in start.. {
        if check(&md5(format!("{input}{i}"))) {
            return i;
        }
    }
    unreachable!()
}

fn part1(input: &str) -> u64 {
    let check_5_zeros = |bytes: &[u8]| matches!(bytes, [0, 0, 0..=0x0F, ..]);
    find_suffix(input, 0, check_5_zeros)
}

fn part2(input: &str, start: u64) -> u64 {
    let check_6_zeros = |bytes: &[u8]| matches!(bytes, [0, 0, 0, ..]);
    find_suffix(input, start, check_6_zeros)
}

const INPUT: &str = include_str!("../../../inputs/2015/04.txt");

fn main() {
    let input = parse(INPUT);

    let part1_res = part1(input);
    println!("Part 1: {part1_res}");
    println!("Part 2: {}", part2(input, part1_res));
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("abcdef", 609043)]
    #[case("pqrstuv", 1048970)]
    fn test_part1(#[case] input: &str, #[case] expected: u64) {
        assert_eq!(part1(input), expected)
    }
}

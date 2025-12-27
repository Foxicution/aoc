use std::collections::HashMap;

fn parse(input: &str) -> &str {
    input.trim()
}

fn part1(input: &str) -> usize {
    input
        .lines()
        .filter(|l| {
            let mut vowels: u8 = 0;
            let mut same_letters = false;
            let bytes = l.as_bytes();

            if bytes.len() < 3 {
                return false;
            }

            if b"aeiou".contains(&bytes[0]) {
                vowels += 1;
            }

            for w in l.as_bytes().windows(2) {
                if w == b"ab" || w == b"cd" || w == b"pq" || w == b"xy" {
                    return false;
                }

                if !same_letters && w[0] == w[1] {
                    same_letters = true;
                }

                if b"aeiou".contains(&w[1]) && vowels < 3 {
                    vowels += 1;
                }
            }
            vowels == 3 && same_letters
        })
        .count()
}

fn part2(input: &str) -> usize {
    input
        .lines()
        .filter(|l| {
            let mut pairs: HashMap<(u8, u8), usize> = HashMap::new();
            let mut pair_match = false;
            let mut letter_match = false;
            let bytes = l.as_bytes();

            pairs.insert((bytes[0], bytes[1]), 0);

            for i in 1..l.len() - 1 {
                if !letter_match && bytes[i - 1] == bytes[i + 1] {
                    letter_match = true;
                }
                if !pair_match {
                    if let Some(pos) = pairs.get(&(bytes[i], bytes[i + 1])) {
                        if pos + 2 <= i {
                            pair_match = true;
                        }
                    } else {
                        pairs.insert((bytes[i], bytes[i + 1]), i);
                    }
                }

                if pair_match && letter_match {
                    return true;
                }
            }
            false
        })
        .count()
}

const INPUT: &str = include_str!("../../../inputs/2015/05.txt");

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
    #[case("ugknbfddgicrmopn", 1)]
    #[case("aaa", 1)]
    #[case("jchzalrnumimnmhp", 0)]
    #[case("haegwjzuvuyypxyu", 0)]
    #[case("dvszwmarrgswjxmb", 0)]
    #[trace]
    fn test_part1(#[case] input: &str, #[case] expected: usize) {
        assert_eq!(part1(parse(input)), expected)
    }

    #[rstest]
    #[case("aaa", 0)]
    #[case("qjhvhtzxzqqjkmpb", 1)]
    #[case("xxyxx", 1)]
    #[case("uurcxstgmygtbstg", 0)]
    #[case("ieodomkazucvgmuy", 0)]
    #[trace]
    fn test_part2(#[case] input: &str, #[case] expected: usize) {
        assert_eq!(part2(input), expected)
    }
}

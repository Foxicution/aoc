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

fn part2(input: &str) -> u8 {
    0
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
        assert_eq!(part1(input), expected)
    }

    // #[rstest]
    // #[case("", 1)]
    // fn test_part2(#[case] input: &str, #[case] expected: u8) {
    //     assert_eq!(part2(input), expected)
    // }
}

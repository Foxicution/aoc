fn parse(input: &str) -> &[u8] {
    input.trim().as_bytes()
}

fn part1(input: &[u8]) -> u32 {
    let mut count = 0;
    let mut i = 0;

    while i < input.len() {
        match input[i] {
            b'"' => count += 1,
            b'\\' => {
                i += 1;
                match input[i] {
                    b'x' => {
                        i += 2;
                        count += 3;
                    }
                    _ => count += 1,
                }
            }
            _ => {}
        }
        i += 1;
    }

    count
}

fn part2(input: &[u8]) -> u32 {
    2 + input
        .iter()
        .map(|ch| match ch {
            b'\\' | b'"' => 1,
            b'\n' => 2,
            _ => 0,
        })
        .sum::<u32>()
}

const INPUT: &str = include_str!("../../../inputs/2015/08.txt");

fn main() {
    let input = parse(INPUT);

    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    const INPUT: &str = r#"
""
"abc"
"aaa\"aaa"
"\x27"
"#;

    #[rstest]
    fn test_part1() {
        println!("{}", INPUT.trim());
        assert_eq!(part1(parse(INPUT)), 12)
    }

    #[rstest]
    fn test_part2() {
        assert_eq!(part2(parse(INPUT)), 19)
    }
}

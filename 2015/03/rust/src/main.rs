fn part1(input: &str) -> usize {
    let mut pos = (0, 0);
    let mut visited = std::collections::HashSet::new();
    visited.insert(pos);

    for ch in input.bytes() {
        match ch {
            b'^' => pos.1 += 1,
            b'v' => pos.1 -= 1,
            b'>' => pos.0 += 1,
            b'<' => pos.0 -= 1,
            _ => unreachable!(),
        };

        visited.insert(pos);
    }

    visited.len()
}

fn part2(input: &str) -> usize {
    let mut real_pos = (0, 0);
    let mut robo_pos = (0, 0);
    let mut real = true;

    let mut visited = std::collections::HashSet::new();
    visited.insert(real_pos);

    for ch in input.bytes() {
        if real {
            match ch {
                b'^' => real_pos.1 += 1,
                b'v' => real_pos.1 -= 1,
                b'>' => real_pos.0 += 1,
                b'<' => real_pos.0 -= 1,
                _ => unreachable!(),
            };
            visited.insert(real_pos);
            real = false;
        } else {
            match ch {
                b'^' => robo_pos.1 += 1,
                b'v' => robo_pos.1 -= 1,
                b'>' => robo_pos.0 += 1,
                b'<' => robo_pos.0 -= 1,
                _ => unreachable!(),
            };
            visited.insert(robo_pos);
            real = true;
        }
    }

    visited.len()
}

fn main() {
    let input = include_str!("../../input.txt");

    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(">", 2)]
    #[case("^>v<", 4)]
    #[case("^v^v^v^v^v", 2)]
    fn test_part1(#[case] input: &str, #[case] expected: usize) {
        assert_eq!(part1(input), expected)
    }

    #[rstest]
    #[case("^v", 3)]
    #[case("^>v<", 3)]
    #[case("^v^v^v^v^v", 11)]
    fn test_part2(#[case] input: &str, #[case] expected: usize) {
        assert_eq!(part2(input), expected)
    }
}

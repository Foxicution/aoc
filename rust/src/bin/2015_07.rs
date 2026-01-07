use std::collections::HashMap;

#[derive(Clone, Debug)]
enum Val {
    Num(u16),
    Wire(String),
}

#[derive(Clone, Debug)]
enum Gate {
    Eq(Val),
    Not(Val),
    And(Val, Val),
    Or(Val, Val),
    LShift(Val, u16),
    RShift(Val, u16),
}

type Circuit = HashMap<String, Gate>;
type Cache = HashMap<String, u16>;

fn parse_val(s: &str) -> Val {
    match s.parse::<u16>() {
        Ok(n) => Val::Num(n),
        Err(_) => Val::Wire(s.to_string()),
    }
}

fn parse(input: &str) -> Circuit {
    input
        .trim()
        .lines()
        .map(|line| {
            let (expr, wire) = line.split_once(" -> ").unwrap();
            let parts: Vec<&str> = expr.split_whitespace().collect();
            let gate = match parts.as_slice() {
                [a] => Gate::Eq(parse_val(a)),
                ["NOT", a] => Gate::Not(parse_val(a)),
                [a, "AND", b] => Gate::And(parse_val(a), parse_val(b)),
                [a, "OR", b] => Gate::Or(parse_val(a), parse_val(b)),
                [a, "LSHIFT", n] => Gate::LShift(parse_val(a), n.parse().unwrap()),
                [a, "RSHIFT", n] => Gate::RShift(parse_val(a), n.parse().unwrap()),
                _ => unreachable!(),
            };

            (wire.to_string(), gate)
        })
        .collect()
}

fn resolve(wire: &str, circuit: &Circuit, cache: &mut Cache) -> u16 {
    if let Some(&val) = cache.get(wire) {
        return val;
    }

    let mut eval = |val: &Val| -> u16 {
        match val {
            Val::Num(n) => *n,
            Val::Wire(w) => resolve(w, circuit, cache),
        }
    };

    let gate = circuit
        .get(wire)
        .unwrap_or_else(|| panic!("Value {wire} should exist"));

    let result = match gate {
        Gate::Eq(val) => eval(val),
        Gate::Not(val) => !eval(val),
        Gate::And(a, b) => eval(a) & eval(b),
        Gate::Or(a, b) => eval(a) | eval(b),
        Gate::LShift(a, n) => eval(a) << n,
        Gate::RShift(a, n) => eval(a) >> n,
    };

    cache.insert(wire.to_string(), result);
    result
}

fn part1(input: &Circuit, cache: &mut Cache) -> u16 {
    resolve("a", input, cache)
}

fn part2(input: &Circuit, cache: &mut Cache) -> u16 {
    resolve("a", input, cache)
}

const INPUT: &str = include_str!("../../../inputs/2015/07.txt");

fn main() {
    let input = parse(INPUT);
    let mut cache = HashMap::new();

    let p1_result = part1(&input, &mut cache);
    println!("Part 1: {}", p1_result);

    cache.clear();
    cache.insert("b".to_string(), p1_result);
    println!("Part 2: {}", part2(&input, &mut cache));
}

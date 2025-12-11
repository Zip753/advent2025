use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

fn parse_line(line: &str) -> (u16, usize, Vec<u16>, Vec<u32>) {
    // Parse [...]
    let bracket_start = line.find('[').expect("missing '[' in line");
    let bracket_end = line.find(']').expect("missing ']' in line");
    let target_str = &line[bracket_start + 1..bracket_end];

    let target_bits = target_str.len();
    let mut target: u16 = 0;
    for (i, c) in target_str.chars().enumerate() {
        if c == '#' {
            target |= 1 << i;
        }
    }

    // Parse (...) entries
    let mut actions: Vec<u16> = Vec::new();
    let mut rest = &line[bracket_end + 1..];

    while let Some(paren_start) = rest.find('(') {
        let paren_end = rest.find(')').expect("missing ')' after '('");
        let action_str = &rest[paren_start + 1..paren_end];

        let mut action: u16 = 0;
        for num_str in action_str.split(',') {
            let bit: usize = num_str.trim().parse().expect("invalid action bit");
            action |= 1 << bit;
        }
        actions.push(action);

        rest = &rest[paren_end + 1..];
        if rest.trim_start().starts_with('{') {
            break;
        }
    }

    // Parse {...}
    let brace_start = rest.find('{').expect("missing '{' in line");
    let brace_end = rest.find('}').expect("missing '}' in line");
    let cost_str = &rest[brace_start + 1..brace_end];

    let mut costs: Vec<u32> = Vec::new();
    for num_str in cost_str.split(',') {
        let cost: u32 = num_str.trim().parse().expect("invalid cost number");
        costs.push(cost);
    }

    (target, target_bits, actions, costs)
}

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("../input.txt")?;
    let reader = BufReader::new(file);

    let mut results: Vec<u32> = Vec::new();

    for line in reader.lines() {
        let line = line?;
        if line.is_empty() {
            continue;
        }

        let (target, target_bits, actions, _costs) = parse_line(&line);
        let result = switches::min_moves(target, target_bits, &actions);
        results.push(result.expect("should have a solution"));
    }

    eprintln!("{:?}", results);
    println!("{}", results.iter().sum::<u32>());

    Ok(())
}

#[derive(Clone, Copy, Debug)]
struct Lanternfish {
    timer: u8,
}

fn parse_input(input: &str) -> Vec<Lanternfish> {
    input
        .trim()
        .split(',')
        .into_iter()
        .map(|value| Lanternfish {
            timer: value.parse::<u8>().unwrap(),
        })
        .collect::<Vec<Lanternfish>>()
}

fn part_one(fishes: &mut Vec<Lanternfish>, iterations: usize) -> usize {
    for _ in 0..iterations {
        let mut new_fishes: Vec<Lanternfish> = Vec::new();
        fishes.into_iter().for_each(|mut fish| match fish.timer {
            0 => {
                fish.timer = 6;
                new_fishes.push(Lanternfish { timer: 8 })
            }
            _ => fish.timer -= 1,
        });
        fishes.append(&mut new_fishes);
        new_fishes.clear();
    }

    fishes.len()
}

fn part_two(fishes: &mut Vec<Lanternfish>, iterations: usize) -> u128 {
    let mut state_slice: [u128; 10] = [0u128; 10];
    for fish in fishes.iter() {
        state_slice[fish.timer as usize] += 1;
    }
    for _ in 0..iterations {
        state_slice[9] = state_slice[0];
        for state in 1..=8 {
            state_slice[state - 1] = state_slice[state];
        }
        state_slice[6] += state_slice[9];
        state_slice[8] = state_slice[9];
    }
    state_slice[9] = 0;
    state_slice.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_data() {
        let input = "3,4,3,1,2";
        let parsed = parse_input(input);

        assert_eq!(parsed.len(), 5);
    }

    #[test]
    fn test_part_one() {
        let input = "3,4,3,1,2";
        let mut parsed = parse_input(input);

        assert_eq!(part_one(&mut parsed, 18), 26);
    }

    #[test]
    fn test_part_two() {
        let input = "3,4,3,1,2";
        let mut parsed = parse_input(input);

        assert_eq!(part_two(&mut parsed, 256), 26984457539);
    }
}

fn main() {
    let mut fishes = parse_input(include_str!("../input.txt"));
    let mut fishes_part_two = fishes.clone();
    let mut iterations = 80;
    println!("Part 1:");
    println!(
        "fishes after {} iterations: {}",
        iterations,
        part_one(&mut fishes, iterations)
    );

    iterations = 256;
    println!("Part 2:");
    println!(
        "fishes after {} iterations: {}",
        iterations,
        part_two(&mut fishes_part_two, iterations)
    );
}

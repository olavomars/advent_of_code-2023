use std::fs;

fn is_valid_cube(digit: u32, color: &str) -> bool {
    match color {
        "blue" => digit <= 14,
        "green" => digit <= 13,
        "red" => digit <= 12,
        _ => false,
    }
}

fn is_valid_set(set: &str) -> bool {
    set.split(", ").all(|cube| {
        let (digit, color) = cube.split_once(" ").unwrap();
        let digit = digit.parse::<u32>().unwrap();
        is_valid_cube(digit, color)
    })
}

fn is_valid_game(line: &str) -> Option<u32> {
    let (game, rest) = line.split_once(": ")?;
    let id = game.strip_prefix("Game ")?.parse::<u32>().ok()?;

    if rest.split("; ").all(is_valid_set) {
        Some(id)
    } else {
        None
    }
}

fn part1(input: &str) -> u32 {
    let sum = input.lines().filter_map(is_valid_game).sum();

    println!("Sum: {}", sum);
    sum
}

fn part2(input: &str) -> u32 {
    let sum = input.lines().map(|line| {
        let (_, rest) = line.split_once(": ").unwrap();

        let mut max_blue = 0;
        let mut max_green = 0;
        let mut max_red = 0;

        rest.split("; ").for_each(|set| {
            set.split(", ").for_each(|cube| {
                let (digit, color) = cube.split_once(" ").unwrap();
                let digit = digit.parse::<u32>().unwrap();

                match color {
                    "blue" => max_blue = max_blue.max(digit),
                    "green" => max_green = max_green.max(digit),
                    "red" => max_red = max_red.max(digit),
                    _ => {},
                }
            });
        });

        max_blue * max_green * max_red
    }).sum::<u32>();

    println!("Sum: {}", sum);
    sum
}


fn main() -> std::io::Result<()> {
    let input = fs::read_to_string("input.txt")?;
    part1(&input);
    part2(&input);

    Ok(())
}

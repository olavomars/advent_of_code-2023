use std::collections::HashMap;

const INDEX_MAX: usize = 139;

type Key = (usize, usize);

type Map = HashMap<Key, String>;

fn insert_number(key: Key, num: &str, map: &mut Map) {
    for x in 0..num.len() {
        map.insert((key.0, key.1 - x), num.to_string());
    }
}

fn look_around(key: &Key, map: &Map) -> Option<Vec<u32>> {
    // is my row within my set boundaries for my string?
    assert!(key.0 > 0);
    assert!(key.0 < INDEX_MAX);
    let (row, index) = *key;
    let mut result = vec![];
    
    // check for numbers on left or right
    for i in [0, 2] {
        if let Some(x) = map.get(&(row, index + i - 1)) {
            if let Ok(num) = x.parse::<u32>() {
                result.push(num);
            }
        }
    }
    
    // check for numbers above or below
    for r in [0, 2] {
        for i in [1, 0, 2] {
            if let Some(x) = map.get(&(row + r - 1, index + i - 1)) {
                if let Ok(num) = x.parse::<u32>() {
                    result.push(num);
                    // Skip to the next row if we are at the center column.
                    if i == 1 {
                        break;
                    }
                }
            }
        }
    }

    if result.is_empty() {
        None
    } else {
        Some(result)
    }
}

fn result(data: &str) -> (u32, u32) {
     let mut map = Map::new();

     for (row, data_line) in data.lines().enumerate() {
         let mut num = String::new();

        for (i, ch) in data_line.char_indices() {
            match ch {
                // period
                '.' => {
                     if !num.is_empty() {
                        insert_number((row, i - 1), &num, &mut map);
                        num.clear();
                     }
                },
                // numbers
                n if n.is_ascii_digit() => {
                    num.push(n);
                    // If this is the last character in the row, insert the current number.
                    if i == INDEX_MAX {
                        insert_number((row, i), &num, &mut map);
                        num.clear();
                    }
                },
                // everything else is gonna be special character
                _ => {
                    if !num.is_empty() {
                        insert_number((row, i - 1), &num, &mut map);
                        num.clear();
                    }
                    map.insert((row, i), ch.to_string());
                },
            }
        }
     }
    let mut sum = 0;
    let mut gears = 0;

     for (k, v) in map.iter() {
        match v.as_str() {
            // Ignore digits
            n if n.parse::<u32>().is_ok() => (),

            // If the character is an asterisk, we look around it for numbers to sum.
            "*" => if let Some(numbers) = look_around(k, &map) {
                sum += numbers.iter().sum::<u32>();
                // If there are exactly two numbers, they represent gear teeth, so multiply them hoes
                if numbers.len() == 2 {
                    gears += numbers[0] * numbers[1]
                }
            },

            // For all other special characters, only sum the numbers around it.
            _ => if let Some(numbers) = look_around(k, &map) {
                sum += numbers.iter().sum::<u32>();
            },
        }
    }
    (sum, gears)
}

fn main() {
    let (phase1, phase2) = result(include_str!("input.txt"));
    println!("Answer day 3.1 = {}", phase1);
    println!("Answer day 3.2 = {}", phase2);
}
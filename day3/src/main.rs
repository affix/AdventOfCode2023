use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn find_integers(map: &[&str]) -> Vec<(String, usize, usize)> {
    let mut integers = vec![];

    for (y, row) in map.iter().enumerate() {
        let mut x = 0;
        while x < row.len() {
            if row.chars().nth(x).unwrap().is_digit(10) {
                let mut num_str = String::new();
                while x < row.len() && row.chars().nth(x).unwrap().is_digit(10) {
                    num_str.push(row.chars().nth(x).unwrap());
                    x += 1;
                }
                integers.push((num_str.clone(), x - num_str.len(), y));
            } else {
                x += 1;
            }
        }
    }

    integers
}

fn is_near_symbol(map: &[&str], num_str: &str, start_x: usize, y: usize) -> bool {
    for (i, _) in num_str.chars().enumerate() {
        let x = start_x + i;
        for dx in -1..=1 {
            for dy in -1..=1 {
                if dx == 0 && dy == 0 { continue; }
                if let Some(row) = map.get((y as isize + dy) as usize) {
                    if let Some(ch) = row.chars().nth((x as isize + dx) as usize) {
                        if ch != '.' && ch != ' ' && !ch.is_numeric() {
                            return true;
                        }
                    }
                }
            }
        }
    }
    false
}

fn find_symbol(lines: &[&[u8]], r: usize, c: usize) -> Option<(usize, usize, char)> {
  for (dr, dc) in [(-1,-1),(-1,0),(-1,1),(0,-1),(0,1),(1,-1),(1,0),(1,1)] {
    let (rr, cc) = ((r as i32 + dr) as usize, (c as i32 + dc) as usize);
    let Some(&s) = lines.get(rr).and_then(|l| l.get(cc)) else {continue};
    if s != b'.' && !s.is_ascii_digit() {
      return Some((cc, rr, s as char));
    }
  }
  None
}

use std::collections::HashMap;

fn get_symbols(lines: &[&[u8]]) -> HashMap<(usize, usize, char), Vec<usize>> {
    let mut symbols = HashMap::new();
    for (r, l) in lines.iter().enumerate() {
        let mut c = 0;
        while c < l.len() {
        let (start, mut symbol) = (c, None);
        while c < l.len() && l[c].is_ascii_digit() {
            symbol = symbol.or_else(|| find_symbol(&lines, r, c));
            c += 1;
        }
        if let Some(symbol) = symbol {
            let num = l[start..c].iter().fold(0, |n, c| n * 10 + (c - b'0') as usize);
            symbols.entry(symbol).or_insert(Vec::new()).push(num);
        }
        c += 1;
        }
    }
    symbols
}

fn read_lines<P>(filename: P) -> io::Result<Vec<String>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    let buf_reader = io::BufReader::new(file);
    buf_reader.lines().collect() // Collects lines into a Vec<String>
}

fn main() {
    let map = read_lines("input").unwrap();
    let map: Vec<&str> = map.iter().map(|s| s.as_str()).collect();
    let integers = find_integers(&map);
    let mut result = 0;
    for (num, x, y) in integers {
        if is_near_symbol(&map, &num, x, y) {
            result += &num.parse::<i32>().unwrap();
        }
    }
    println!("Day 3 Part 1 : {}", result);


    let lines = include_str!("../input");
    let lines = lines.split("\n").map(str::as_bytes).collect::<Vec<_>>();

    let symbols = get_symbols(&lines);

    let p2: usize = symbols.iter()
        .filter(|(&(_, _, s), v)| s == '*' && v.len() == 2)
        .map(|(_, v)| v[0] * v[1])
        .sum();
    
    println!("Day 3 Part 2 : {}", p2);

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day1_map_test() {
        let map = [
            "467..114..",
            "...*......",
            "..35..633.",
            "......#...",
            "617*......",
            ".....+.58.",
            "..592.....",
            "......755.",
            "...$.*....",
            ".664.598..",
        ];

        let results = [
            true,
            false,
            true,
            true,
            true,
            false,
            true,
            true,
            true,
            true
        ];

        let integers = find_integers(&map);
        for (i, num) in integers.iter().enumerate() {
            assert_eq!(is_near_symbol(&map, &num.0, num.1, num.2), results[i]);
        }
    }
}
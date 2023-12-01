fn calibrate_line_part1(line: &str) -> i32 {
     let digits: Vec<i32> = line.chars()
        .filter_map(|c| c.to_digit(10))
        .map(|d| d as i32)
        .collect();
    let value: i32 = format!("{:?}{:?}", digits.first().unwrap(), digits.last().unwrap()).parse().unwrap();
    return value;
}

fn str_to_char(val: &str) -> Option<char> {
    if val.starts_with("one") || val.starts_with('1') {
        Some('1')
    } else if val.starts_with("two") || val.starts_with('2') {
        Some('2')
    } else if val.starts_with("three") || val.starts_with('3') {
        Some('3')
    } else if val.starts_with("four") || val.starts_with('4') {
        Some('4')
    } else if val.starts_with("five") || val.starts_with('5') {
        Some('5')
    } else if val.starts_with("six") || val.starts_with('6') {
        Some('6')
    } else if val.starts_with("seven") || val.starts_with('7') {
        Some('7')
    } else if val.starts_with("eight") || val.starts_with('8') {
        Some('8')
    } else if val.starts_with("nine") || val.starts_with('9') {
        Some('9')
    } else {
        None
    }
}

fn get_single_digit(val: &str) -> Option<char> {
    if val.is_empty() {
        return None;
    }
    match str_to_char(val) {
        Some(c) => Some(c),
        None => {
            get_single_digit(&val[1..])
        }
    }
}

fn calibrate_line_part2(line: &str) -> i32 {
    let first = get_single_digit(line).unwrap();
    let mut sub = &line[1..];
    let mut last = first;
    while !sub.is_empty() {
        if let Some(c) = str_to_char(sub){
            last = c;
        }
        sub = &sub[1..];
    }
    format!("{first}{last}").parse().unwrap()
}

fn main() {
    let lines = include_str!("../input").lines();
    println!("Part 1 : {}", lines.map(|line| calibrate_line_part1(&line)).sum::<i32>().to_string());

    let lines = include_str!("../input").lines();
    println!("Part 2 : {}", lines.map(|line| calibrate_line_part2(&line)).sum::<i32>().to_string());
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calibrate_line_part1() {
        let mut lines = HashMap::new();

        lines.insert("1abc2", 12);
        lines.insert("pqr3stu8vwx", 38);
        lines.insert("a1b2c3d4e5f", 15);
        lines.insert("treb7uchet", 77);
        


        for (line, expected) in lines {
            let actual = calibrate_line_part1(line);
            assert_eq!(expected, actual);
        }
    }

    #[test]
    fn test_calibrate_line_part2() {
        let mut lines = HashMap::new();

        lines.insert("two1nine", 29);
        lines.insert("eightwothree", 83);
        lines.insert("abcone2threexyz", 13);
        lines.insert("xtwone3four", 24);
        lines.insert("4nineeightseven2", 42);
        lines.insert("zoneight234", 14);
        lines.insert("7pqrstsixteen",76);

        let mut values = Vec::<i32>::new();
        
        for (line, expected) in lines {
            let actual = calibrate_line_part2(line);
            assert_eq!(expected, actual);
            values.push(actual);
        }
      
        let sum = values.iter().sum::<i32>();
        assert_eq!(sum, 281);
    }
}
fn main() {
    let input: &str = include_str!("data/input2.txt");
    let output: String = part2(input);
    dbg!(output);
}

fn part2(input: &str) -> String {
    let mut count: i32 = 0;
    let mut enabled: bool = true;

    for line in input.lines() {
        for (index, _) in line.chars().enumerate() {
            if index + 7 <= line.len() && &line[index..index + 7] == "don't()" {
                enabled = false;
            } else if index + 4 <= line.len() && &line[index..index + 4] == "do()" {
                enabled = true;
            }

            if index + 4 <= line.len() && &line[index..index + 4] == "mul(" && enabled {
                let end_index: usize = line[index + 4..].find(')').unwrap_or(0) + index + 4;
                let content: &str = &line[index + 4..end_index];
                if content.split(',').all(|s: &str| s.trim().parse::<i32>().is_ok()) {
                    let mut iter: std::str::Split<'_, char> = content.split(',');
                    let num: i32 = iter.next().unwrap().trim().parse::<i32>().unwrap();
                    let num2: i32 = iter.next().unwrap().trim().parse::<i32>().unwrap();
                    count += num * num2;
                }
            }
        }
    }

    count.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let input: &str = include_str!("data/test_input2.txt");
        let output: String = part2(input);
        assert_eq!(output, "48".to_string());
    }
}

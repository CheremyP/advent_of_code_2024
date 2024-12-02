fn main(){
    let input: &str = include_str!("data/input1.txt");
    let output =  part1(input);
    dbg!(output);
}

fn part1(input: &str) -> String {
    let mut count: i32 = 0;

    for line in input.lines() {
        let levels: Vec<i32> = line.split_whitespace()
                                   .map(|s| s.parse().unwrap())
                                   .collect();

        if levels.len() < 2 {
            continue;
        }

        let mut is_increasing = true;
        let mut is_decreasing = true;

        for i in 0..levels.len() - 1 {
            let difference = (levels[i] - levels[i + 1]).abs();

            if difference < 1 || difference > 3 {
                is_increasing = false;
                is_decreasing = false;
                break;
            }

            if levels[i] < levels[i + 1] {
                is_decreasing = false;
            } else if levels[i] > levels[i + 1] {
                is_increasing = false;
            }
        }

        if is_increasing || is_decreasing {
            count += 1;
        }
    }

    count.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let input:&str = include_str!("data/test_input1.txt");
        let output = part1(input);
        assert_eq!(output, "2".to_string());
    }
}
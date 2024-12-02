fn main(){
    let input: &str = include_str!("data/input2.txt");
    let output =  part2(input);
    dbg!(output);
}

fn part2(input:&str)-> String{
    let mut count: i32 = 0;

    for line in input.lines() {
        let levels: Vec<i32> = line.split_whitespace()
                                   .map(|s| s.parse().unwrap())
                                   .collect();

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
            continue;
        }

        for i in 0..levels.len(){
            let mut modified_levels = levels.clone();
            modified_levels.remove(i);
            let mut is_increasing = true;
            let mut is_decreasing = true;
            
            for j in 0..modified_levels.len() - 1 {
                let difference = (modified_levels[j] - modified_levels[j + 1]).abs();

                if difference < 1 || difference > 3 {
                    is_increasing = false;
                    is_decreasing = false;
                    break;
                }

                if modified_levels[j] < modified_levels[j + 1] {
                    is_decreasing = false;
                } else if modified_levels[j] > modified_levels[j + 1] {
                    is_increasing = false;
                }
            }
            if is_increasing || is_decreasing {
                count += 1;
                break;
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
        let input:&str = include_str!("data/test_input2.txt");
        let output = part2(input);
        assert_eq!(output, "4".to_string());
    }
}
use core::num;

fn main(){
    let input: &str = include_str!("data/input1.txt");
    let output =  part1(input);
    dbg!(output);
}

fn part1(input:&str)-> String{
    let mut count: i32 = 0;
    for line in input.lines(){
        for (index, _) in line.chars().enumerate() {
            if index + 4 <= line.len() && &line[index..index + 4] == "mul(" {
                let end_index: usize = line[index + 4..].find(')').unwrap_or(index + 4) + index + 4;
                let content: &str = &line[index + 4..end_index];
                let mut num: i32 = 0;
                let mut num2: i32 = 0;  
                if content.split(',').all(|s| s.trim().parse::<i32>().is_ok()) {
                    let mut iter = content.split(',');
                    num = iter.next().unwrap().trim().parse::<i32>().unwrap();
                    num2 = iter.next().unwrap().trim().parse::<i32>().unwrap();
                    count+= num * num2;
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
        let input:&str = include_str!("data/test_input1.txt");
        let output = part1(input);
        assert_eq!(output, "161".to_string());
    }
}
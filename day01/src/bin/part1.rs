fn main(){
    let input: &str = include_str!("data/input1.txt");
    let output = part1(input);

    dbg!(output);
}

fn part1(input: &str) -> String {
    let mut list1: Vec<i32> = Vec::new();
    let mut list2: Vec<i32> = Vec::new();
    let mut parts: Vec<&str>;
    
    for line in input.lines() {
        parts = line.split_whitespace().collect::<Vec<&str>>();
        if parts.len() == 1 {
            // If there's only one value, push it to both lists
            let value = parts[0].parse::<i32>().expect("Failed to parse integer");
            list1.push(value);
            list2.push(value);
        } else if parts.len() == 2 {
            // If there are two values, push them to the respective lists
            list1.push(parts[0].parse::<i32>().expect("Failed to parse integer"));
            list2.push(parts[1].parse::<i32>().expect("Failed to parse integer"));
        } else {
            panic!("Each line must contain one or two space-separated values");
        }
    }
    
    // Sort the lists
    list1.sort();
    list2.sort();

    // Find the difference between the sorted values
    let mut sum: i32 = 0;
    for i in 0..list1.len() {
        sum += (list2[i] - list1[i]).abs();
    }
    return sum.to_string()
    
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let input: &str = include_str!("data/test_input1.txt");
        let output = part1(input);
        println!("{}", output);
        assert_eq!(output, "11".to_string());
    }
}
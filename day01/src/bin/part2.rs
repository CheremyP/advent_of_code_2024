use std::collections::HashMap;

fn main(){
    let input: &str = include_str!("data/input2.txt");
    let output =  part2(input);
    dbg!(output);
}

fn part2(input:&str)-> String{
    let mut list1: Vec<i32> = Vec::new();
    let mut list2: Vec<i32> = Vec::new();
    let mut parts: Vec<&str>;
    let mut similarity_score: i32 = 0;

    for line in input.lines(){
        parts = line.split_whitespace().collect::<Vec<&str>>();
        if parts.len() == 1 {
            let value = parts[0].parse::<i32>().expect("Failed to parse integer");
            list1.push(value);
            list2.push(value);
        } else if parts.len() == 2 {
            list1.push(parts[0].parse::<i32>().expect("Failed to parse integer"));
            list2.push(parts[1].parse::<i32>().expect("Failed to parse integer"));
        } else {
            panic!("Each line must contain one or two space-separated values");
        }
    }

    let mut map1: HashMap<i32, i32> = HashMap::new();
    for i in 0..list1.len() {
        map1.insert(list1[i], 0);
    }
    for i in list2 {
        if map1.contains_key(&i) {
            map1.insert(i, map1.get(&i).unwrap() + 1);
        }
    }

    // calculate the similarity score
    for i in 0..list1.len() {
        similarity_score += map1.get(&list1[i]).unwrap() * list1[i];

    }

    return similarity_score.to_string();
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let input:&str = include_str!("data/test_input2.txt");
        let output = part2(input);
        assert_eq!(output, "31".to_string());
    }
}
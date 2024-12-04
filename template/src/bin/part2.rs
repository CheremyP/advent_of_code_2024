fn main(){
    let input: &str = include_str!("data/input2.txt");
    let output: String =  part2(input);
    dbg!(output);
}

fn part2(input:&str)-> String{
    for line in input.lines(){
        println!("{}", line);
    }
    todo!()
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let input:&str = include_str!("data/test_input2.txt");
        let output: String = part2(input);
        assert_eq!(output, "142".to_string());
    }
}
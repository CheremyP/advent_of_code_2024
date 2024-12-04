fn main(){
    let input: &str = include_str!("data/input1.txt");
    let output: String =  part1(input);
    dbg!(output);
}

fn part1(input:&str)-> String{
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
        let input:&str = include_str!("data/test_input1.txt");
        let output: String = part1(input);
        assert_eq!(output, "142".to_string());
    }
}
fn main() {
    let input = include_str!("input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> String {
    "todo!()".to_string()
}

//test commmit if i had remove wrong user on git
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let restult = part1("");
        assert_eq!(restult, "4".to_string());
    }
}

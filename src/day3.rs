#[cfg(test)]
mod tests {
    use regex::Regex;
    use std::fs;
    use std::path::Path;

    #[derive(Debug)]
    struct Mul {
        x: i64,
        y: i64
    }

    #[test]
    fn part1() {
        let string = fs::read_to_string(Path::new("./src/input/day-03.in")).unwrap();
        let regex = Regex::new("mul\\(([0-9]{1,3}),([0-9]{1,3})\\)").unwrap();
        let result: i64 = regex.captures_iter(string.as_str())
            .map(|m| Mul { x: m[1].parse().unwrap(), y: m[2].parse().unwrap() })
            .map(|m| m.x * m.y)
            .sum();
        println!("result: {:?}", result);
    }
}
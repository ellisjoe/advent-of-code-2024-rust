#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    use std::path::Path;

    #[test]
    fn part1() {
        let reports = read_lines();
        let num_safe = reports.iter().map(|x| is_safe(x)).filter(|x| *x).count();
        println!("numSafe: {}", num_safe);
    }

    #[test]
    fn part2() {
        let reports = read_lines();
        let num_safe = reports.iter().map(|x| is_safe2(x)).filter(|x| *x).count();
        println!("numSafe: {}", num_safe);
    }

    fn read_lines() -> Vec<Vec<i32>> {
        let path = Path::new("./src/input/day-02.in");
        let file = File::open(path).unwrap();
        let reader = BufReader::new(file);
        reader.lines()
            .map(|l| l.unwrap().split(" ")
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<i32>>())
            .collect::<Vec<Vec<i32>>>()
    }

    fn is_safe2(report: &Vec<i32>) -> bool {
        if is_safe(&report) {
            true
        } else {
            for i in 0..report.len() {
                let mut subreport = report.clone();
                subreport.remove(i);
                if is_safe(&subreport) {
                    return true;
                }
            }
            false
        }
    }

    fn is_safe(report: &Vec<i32>) -> bool {
        let all_ascending = report.windows(2).all(|w| w[0] < w[1]);
        let all_descending = report.windows(2).all(|w| w[0] > w[1]);
        let within_range = report.windows(2).all(|w| withinRange(w[0], w[1]));
        (all_ascending || all_descending) && within_range
    }

    fn withinRange(left: i32, right: i32) -> bool {
        let diff = left.abs_diff(right);
        1 <= diff && diff <= 3
    }
}
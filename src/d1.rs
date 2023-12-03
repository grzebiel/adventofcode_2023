use std::io::{BufRead, BufReader};
use std::fs::File;

// fn extract_elves_summed_callories() -> Vec<i64>
// {
//     let file_reader = BufReader::new(File::open("input/d1.txt").unwrap());
//     file_reader
//         .lines()
//         .map(|line| line.unwrap().parse::<i64>().unwrap_or(-1))
//         .collect::<Vec<i64>>()
//         .split(|i| *i == -1)
//         .map(|single| single.iter().sum())
//         .collect()

// }

pub fn p1(input: &str) -> i64
{

    let file_reader = BufReader::new(File::open(input).unwrap());
    file_reader
        .lines()
        .map(|line| {
            let l = line.unwrap();
            let first = l
                .chars()
                .find(|c| c.is_digit(10)).unwrap();
            let last = l
                .chars()
                .rev()
                .find(|c| c.is_digit(10)).unwrap();
            format!("{}{}", first, last).parse::<i64>().unwrap()
        })
        .sum()
}

pub fn p2(input: &str) -> i64
{

    let file_reader = BufReader::new(File::open(input).unwrap());
    file_reader
        .lines()
        .map(|line| {
            let l = line
                .unwrap()
                .replace("one", "one1one")
                .replace("two", "two2two")
                .replace("three", "three3three")
                .replace("four", "four4four")
                .replace("five", "five5five")
                .replace("six", "six6six")
                .replace("seven", "seven7seven")
                .replace("eight", "eight8eight")
                .replace("nine", "nine9nine");

            let first = l
                .chars()
                .find(|c| c.is_digit(10)).unwrap();
            let last = l
                .chars()
                .rev()
                .find(|c| c.is_digit(10)).unwrap();
            format!("{}{}", first, last).parse::<i64>().unwrap()
        })
        .sum()
}

#[cfg(test)]
mod test {
    use crate::d1::{p1, p2};

    #[test]
    fn p1_works() {
        assert_eq!(p1("input/d1_test.txt"), 142);
    }

    #[test]
    fn p2_works() {
        assert_eq!(p2("input/d1.txt"), 281);
    }
}

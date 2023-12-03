use std::io::{BufRead, BufReader};
use std::fs::File;

pub fn p1(input: &str) -> i64
{

    let file_reader = BufReader::new(File::open(input).unwrap());
    file_reader
        .lines()
        .map(|line| {
            let l = line.unwrap();
            let game = l.split(": ").collect::<Vec<&str>>();
            let id = game[0].split(" ").collect::<Vec<&str>>()[1].parse::<i64>().unwrap();
            let mut r = 0;
            let mut g = 0;
            let mut b = 0;
            (game[1]);
            game[1]
                .split("; ")
                .for_each(|round| {
                    let _ = round.split(", ").for_each(
                        |res| {

                            let rr = res.split(" ").collect::<Vec<&str>>();
                            let val = rr[0].parse::<i64>().unwrap();
                            match rr[1] {
                                "red" => r = i64::max(r, val),
                                "blue" => b = i64::max(b, val),
                                "green" => g = i64::max(g, val),
                                _ => panic!("this shall never happen"),
                            };
                        });
                });

            (id, r,g,b)
        })
    .filter(|(_, r,g,b)| ((*r)< 13 && (*g) <14 && (*b)<15))
        .map(|(id, _, _, _)| (id))
        .sum()
}

pub fn p2(input: &str) -> i64
{
    let file_reader = BufReader::new(File::open(input).unwrap());
    file_reader
        .lines()
        .map(|line| {
            let l = line.unwrap();
            let game = l.split(": ").collect::<Vec<&str>>();
            let id = game[0].split(" ").collect::<Vec<&str>>()[1].parse::<i64>().unwrap();
            let mut r = 0;
            let mut g = 0;
            let mut b = 0;
            (game[1]);
            game[1]
                .split("; ")
                .for_each(|round| {
                    let _ = round.split(", ").for_each(
                        |res| {

                            let rr = res.split(" ").collect::<Vec<&str>>();
                            let val = rr[0].parse::<i64>().unwrap();
                            match rr[1] {
                                "red" => r = i64::max(r, val),
                                "blue" => b = i64::max(b, val),
                                "green" => g = i64::max(g, val),
                                _ => panic!("this shall never happen"),
                            };
                        });
                    });

            (id, r,g,b)
        })
    .map(|(_, r, g, b)| r*g*b)
        .sum()
}

#[cfg(test)]
mod test {
    use crate::d2::{p1, p2};

    #[test]
    fn p1_works() {
        assert_eq!(p1("input/d2_test.txt"), 8);
    }

    #[test]
    fn p2_works() {
        assert_eq!(p2("input/d2.txt"), 2286);
    }
}

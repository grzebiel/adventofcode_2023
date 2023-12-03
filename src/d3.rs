use std::collections::{VecDeque, HashMap};
use std::io::{BufRead, BufReader};
use std::fs::File;

fn parts_touching(x: usize, y: usize, lines: &Vec<String>) -> bool {
    let mut ret = false;
    let is_part = |p: char| { dbg!(p); !p.is_digit(10) && p !='.' };
    for x in dbg!(x as i64 - 1..x as i64+2) {
        for y in dbg!(y as i64 -1 ..y as i64+2) {
            dbg!(x, y);
            if x >= 0 && x < lines[0].len() as i64 && y >= 0 && y < lines.len() as i64 {
                ret |= dbg!(is_part(lines[y as usize].as_bytes()[x as usize] as char));
            }
        }
    }
    ret
}

pub fn p1(input: &str) -> i64
{

    let file_reader = BufReader::new(File::open(input).unwrap());
    let lines : Vec<String> = file_reader.lines().map(|line| line.unwrap()).collect();

    let mut res = 0;
    for y in 0..lines.len() {
        let l = &lines[y];
        let mut num = String::new();
        let mut is_valid = false;
        for x in 0..l.len()+1 {
            if x < l.len() {
                let c = l.as_bytes()[x] as char;
                if c.is_digit(10){
                    dbg!(x, y, c);
                    num.push((c));
                    if !is_valid {
                        is_valid |= dbg!(parts_touching(x,y, &lines));
                    }

                } else {
                    if is_valid {
                        res += dbg!(num.parse::<i64>().unwrap());
                    }
                    num.clear();
                    is_valid = false;
                }
            } else {
                if is_valid {
                    res += dbg!(num.parse::<i64>().unwrap());
                }
                num.clear();
                is_valid = false;
            }
         }

    }
    res
}

fn parts_touching2(x: usize, y: usize, lines: &Vec<String>) -> (bool, usize, usize) {
    let mut ret = false;
    let is_part = |p: char| { p == '*' };
    for x in (x as i64 - 1..x as i64+2) {
        for y in (y as i64 -1 ..y as i64+2) {
            (x, y);
            if x >= 0 && x < lines[0].len() as i64 && y >= 0 && y < lines.len() as i64 {
                if is_part(lines[y as usize].as_bytes()[x as usize] as char){
                    return dbg!(true, x as usize, y as usize);
                }
            }
        }
    }
    (false, 0, 0)
}

pub fn p2(input: &str) -> i64
{

    let file_reader = BufReader::new(File::open(input).unwrap());
    let lines : Vec<String> = file_reader.lines().map(|line| line.unwrap()).collect();

    let mut gears : HashMap<(usize, usize), Vec<i64>>= HashMap::new();
    for y in 0..lines.len() {
        let l = &lines[y];
        let mut num = String::new();
        let mut is_valid = false;
        let mut vgx = 0;
        let mut vgy = 0;
        for x in 0..l.len()+1 {
            if x < l.len() {
                let c = l.as_bytes()[x] as char;
                if c.is_digit(10){
                    (x, y, c);
                    num.push((c));
                    let (gv, gx, gy)  =  (parts_touching2(x,y, &lines));
                    if gv {
                        dbg!(gx, gy);
                        is_valid = true;
                        vgx = gx;
                        vgy = gy
                    }

                } else {
                    if is_valid {
                        match gears.get_mut(&(vgx, vgy)) {
                            Some(v) => (*v).push(num.parse().unwrap()),
                            None => {gears.insert((vgx, vgy), vec![num.parse().unwrap()]);},
                        }
                    }
                    num.clear();
                    is_valid = false;
                    vgx = 0;
                    vgy = 0;
                }
            } else {
                if is_valid {
                    match gears.get_mut(&(vgx, vgy)) {
                        Some(v) => (*v).push(num.parse().unwrap()),
                        None => {gears.insert((vgx, vgy), vec![num.parse().unwrap()]);},
                    }
                }
                num.clear();
                is_valid = false;
                vgx = 0;
                vgy = 0;
            }
         }

    }

    gears.values().filter( |v| dbg!(v.len()) == 2 ).map( |v| v.iter().product::<i64>() ).sum()
}

#[cfg(test)]
mod test {
    use crate::d3::{p1, p2};

    #[test]
    fn p1_works() {
        assert_eq!(p1("input/d3_test.txt"), 4361);
    }

    #[test]
    fn p2_works() {
        assert_eq!(p2("input/d3.txt"), 467835);
    }
}

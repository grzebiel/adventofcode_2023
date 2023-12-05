use std::collections::VecDeque;
use std::io::{BufRead, BufReader};
use std::fs::File;
use rayon::prelude::*;

pub fn p1(input: &str) -> i64
{

    let file_reader = BufReader::new(File::open(input).unwrap());
    let mut lines = file_reader.lines();
    let seeds = lines.next().unwrap().unwrap().split(": ").skip(1).next().unwrap().split(" ").map(|v| v.parse::<i64>().unwrap()).collect::<Vec::<i64>>();
    let mut convs = Vec::<Vec::<Vec::<i64>>>::new();
    let mut i = 0;
    for l in lines.skip(1){
        let lv = l.unwrap();
        if lv == "" {
            i += 1;
            continue;
        }
        if lv.contains("map") {
            convs.push(Vec::new());
            continue;
        }

        convs[i].push(lv.split(" ").map(|v| v.parse::<i64>().unwrap()).collect())
    }
    seeds.iter().map(|seed|
                     {
                         let mut val = *seed;
                         dbg!(&val);
                         for conv in convs.iter(){
                             for range in conv {
                                 if val >= range[1] && val < (range[1] + range[2]) {
                                     dbg!(range);
                                     val = val + (range[0] - range[1]);
                                     dbg!(&val);
                                     break;
                                 }
                             }
                         }
                         val
                     }).min().unwrap()
}

pub fn p2(input: &str) -> i64
{

    let file_reader = BufReader::new(File::open(input).unwrap());
    let mut lines = file_reader.lines();
    let seeds = lines.next().unwrap().unwrap().split(": ").skip(1).next().unwrap().split(" ").map(|v| v.parse::<i64>().unwrap()).collect::<Vec::<i64>>();
    let mut convs = Vec::<Vec::<Vec::<i64>>>::new();
    let mut i = 0;
    for l in lines.skip(1){
        let lv = l.unwrap();
        if lv == "" {
            i += 1;
            continue;
        }
        if lv.contains("map") {
            convs.push(Vec::new());
            continue;
        }

        convs[i].push(lv.split(" ").map(|v| v.parse::<i64>().unwrap()).collect())
    }
    let mut i = 0;
    seeds.chunks(2)
        .map(|r| {
             dbg!(&i);
             i+=1;
             dbg!(r[0]..r[0]+r[1]).collect::<Vec<i64>>().par_iter()
             .map(|seed|
                  {
                      let mut val = *seed;
                      // dbg!(&val);
                      for conv in convs.iter(){
                          for range in conv {
                              if val >= range[1] && val < (range[1] + range[2]) {
                                  // dbg!(range);
                                  val = val + (range[0] - range[1]);
                                  // dbg!(&val);
                                  break;
                              }
                          }
                      }
                      val
                  }).min().unwrap()
        }).min().unwrap()
}
#[cfg(test)]
mod test {
    use crate::d5::{p1, p2};

    #[test]
    fn p1_works() {
        assert_eq!(p1("input/d5_test.txt"), 35);
        // assert_eq!(p1("input/d4.txt"), 13);
    }

    #[test]
    fn p2_works() {
        assert_eq!(p2("input/d5_test.txt"), 46);
        // assert_eq!(p2("input/d5.txt"), 30); // this heats up the room a bit
    }
}

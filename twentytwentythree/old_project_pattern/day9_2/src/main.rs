use std::env::{self};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

/*
5  10  13  16  21  30  45
  5   3   3   5   9  15
   -2   0   2   4   6
      2   2   2   2
        0   0   0


45  30  21  16  13  10  5
   15  9   5   3   3   5
      6  4   2   0  -2
        2  2   2   2
          0  0  0
*/

#[derive(Debug)]
struct Sequence {
    sequences: Vec<Vec<i32>>,
}

impl Sequence {
    fn process(&mut self) {
        let mut all_zeroes = false;
        let mut current_sequence = self.sequences.get(0).unwrap().clone();
        while !all_zeroes {
            let mut new_sequence = Vec::new();
            for index in 1..current_sequence.len() {
                new_sequence.push(&current_sequence[index - 1] - &current_sequence[index]);
            }
            all_zeroes = new_sequence.clone().into_iter().all(|x| x == 0);
            current_sequence = new_sequence.clone();
            self.sequences.push(new_sequence);
        }
    }

    fn extrapolate(&self) -> i32 {
        let mut current_num = self.sequences.last().unwrap().last().unwrap().clone();
        let mut new_numbers = Vec::new();
        let mut reversed_sequences = self.sequences.clone();
        reversed_sequences.reverse();
        for seq in reversed_sequences {
            let new_num = seq.last().unwrap().clone();
            let next_num_in_seq = new_num - current_num;
            new_numbers.push(next_num_in_seq);
            current_num = next_num_in_seq;
        }
        dbg!(&new_numbers);
        return *new_numbers.last().unwrap();
    }
}

fn line_to_sequence(line: String) -> Sequence {
    let mut first_sequence: Vec<i32> = line.split(" ").map(|s| s.parse().unwrap()).collect();
    first_sequence.reverse();
    dbg!(&first_sequence);
    Sequence {
        sequences: vec![first_sequence],
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let mut sequences = Vec::new();
    if let Ok(lines) = read_lines(file_path) {
        for line in lines {
            if let Ok(ready_line) = line {
                sequences.push(line_to_sequence(ready_line));
            }
        }
    }

    let mut total = 0;
    for mut seq in sequences {
        seq.process();
        total += seq.extrapolate();
        dbg!(total);
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

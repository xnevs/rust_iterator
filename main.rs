use itertools::unfold;
use std::io;
use std::io::BufRead;

struct S {
    words: Vec<String>,
}

struct Line {
    esses: Vec<S>,
}

fn main() {
    let stdin = io::stdin().lock();

    let lines: Vec<Line> = stdin
        .lines()
        .map(|line| Line {
            esses: unfold(line.unwrap().split(' '), |iter| {
                iter.next().map(|x| S {
                    words: iter
                        .by_ref()
                        .take(x.parse().unwrap())
                        .map(String::from)
                        .collect(),
                })
            })
            .collect(),
        })
        .collect();

    lines.iter().for_each(|line| {
        line.esses
            .iter()
            .for_each(|s| s.words.iter().for_each(|word| println!("{}", word)));
        println!();
    });
}

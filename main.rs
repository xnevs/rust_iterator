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

    stdin.lines().for_each(|line| {
        let line = line.unwrap();
        let mut iter = line.split(' ');
        loop {
            match iter.next() {
                Some(x) => {
                    S {
                        words: iter
                            .by_ref()
                            .take(x.parse().unwrap())
                            .map(String::from)
                            .collect(),
                    }
                    .words
                    .iter()
                    .for_each(|x| println!("{}", x));
                }
                None => break,
            }
        }
        println!();
    });
}

use itertools::unfold;
use omnom::BufReadExt;
use std::io;
use std::io::BufRead;
use std::string::FromUtf8Error;

trait ReadStringWhile {
    fn read_string_while<P>(&mut self, predicate: P) -> Result<String, FromUtf8Error>
    where
        P: FnMut(u8) -> bool;
}

impl<T: BufReadExt> ReadStringWhile for T {
    fn read_string_while<P>(&mut self, predicate: P) -> Result<String, FromUtf8Error>
    where
        P: FnMut(u8) -> bool,
    {
        let mut buf: Vec<u8> = Default::default();
        let _ = self.read_while(&mut buf, predicate);
        String::from_utf8(buf)
    }
}

struct S {
    words: Vec<String>,
}

struct Line {
    esses: Vec<S>,
}

fn main() {
    let lines: Vec<Line> = unfold(io::stdin().lock(), |mut r| {
        match r.fill_buf().ok().unwrap_or(&[]) {
            [] => None,
            _ => {
                let line = Line {
                    esses: unfold(&mut r, |r| match r.fill_buf().ok().unwrap_or(&[]) {
                        [] => None,
                        [b'\n', ..] => None,
                        [b, ..] => Some({
                            if *b == b' ' {
                                r.consume(1);
                            }
                            let num: usize = r
                                .read_string_while(|b| b.is_ascii_digit())
                                .unwrap()
                                .parse()
                                .unwrap();
                            println!("{}", num);
                            S {
                                words: unfold(r, |r| {
                                    r.consume(1);
                                    Some(r.read_string_while(|b| !b.is_ascii_whitespace()).unwrap())
                                })
                                .take(num)
                                .collect(),
                            }
                        }),
                    })
                    .collect(),
                };
                r.consume(1);
                Some(line)
            }
        }
    })
    .collect();

    lines.iter().for_each(|line| {
        line.esses
            .iter()
            .for_each(|s| s.words.iter().for_each(|word| println!("{}", word)));
        println!();
    });
}

fn main()
{
    use std::time::Instant;

    let input = include_str!("../input.txt");

    let t = Instant::now();
    let result = doit(input);
    println!("{:?} ({:?})", result, t.elapsed());
}

const ALPHABET: &str = "abcdefghiklmnopqrstuvwxyz";

fn doit(input: &str) -> String
{
    let mut iter = input.lines();
    let (_, key) = iter.next().unwrap().split_once(": ").unwrap();
    iter.next();
    let (_, msg) = iter.next().unwrap().split_once(": ").unwrap();

    let mut grid = vec![];
    for c in key.chars().chain(ALPHABET.chars()) {
        if !grid.contains(&(c as u8)) { grid.push(c as u8) }
    }

    let phrase = msg.split_whitespace()
        .map(|s| {
            let mut word = String::new();
            s.as_bytes()
                .chunks(2)
                .for_each(|c| {
                    let (ch1, ch2) = (c[0], c[1]);
                    let p1 = grid.iter().position(|c| *c == ch1).unwrap();
                    let (r1, c1) = (p1 / 5, p1 % 5);

                    let p2 = grid.iter().position(|c| *c == ch2).unwrap();
                    let (r2, c2) = (p2 / 5, p2 % 5);

                    let (e1, e2) = if r1 == r2 {
                        let e1 = if c1 % 5 == 0 { grid[p1 + 4] } else { grid[p1 - 1] };
                        let e2 = if c2 % 5 == 0 { grid[p2 + 4] } else { grid[p2 - 1] };
                        (e1, e2)
                    } else if c1 == c2 {
                        let e1 = if r1 == 0 { grid[20 + c1] } else { grid[p1 - 5] };
                        let e2 = if r2 == 0 { grid[20 + c2] } else { grid[p2 - 5] };
                        (e1, e2)
                    } else {
                        let e1 = grid[r1 * 5 + c2];
                        let e2 = grid[r2 * 5 + c1];
                        (e1, e2)
                    };

                    word.push(e1 as char);
                    word.push(e2 as char);
                });

            word
        })
        .collect::<Vec<_>>();

    phrase.join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_doit()
    {
        let input = include_str!("../input.txt");
        assert_eq!(doit(input), "please pick up some milk on thex wayx home".to_string());
    }

    #[test]
    fn example_doit()
    {
        let input = include_str!("../example.txt");
        assert_eq!(doit(input), "et phonex home".to_string());
    }
}

fn main()
{
    use std::time::Instant;

    let input = include_str!("../input.txt");

    let t = Instant::now();
    let result = doit(input);
    println!("{} ({:?})", result, t.elapsed());
}

const CHARS: &[u8]  = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789.,;:?! '()";
const SECRET: &[u8] = b"Roads? Where We're Going, We Don't Need Roads.";

fn doit(input: &str) -> String
{
    let secret = SECRET.iter()
        .map(|c1| (*c1, CHARS.iter().position(|c2| c1 == c2).unwrap() + 1))
        .collect::<Vec<_>>();

    let mut iter = secret.iter().cycle();
    input.as_bytes()
        .iter()
        .map(|&c1| {
            if let Some(pos) = CHARS.iter().position(|&c2| c1 == c2) {
                let (_, cnt) = iter.next().unwrap();
                let i = if *cnt > pos {
                    CHARS.len() - (cnt - pos)
                } else {
                    pos - cnt
                };
                CHARS[i] as char
            } else {
                c1 as char
            }
        })
        .collect()
}


#[cfg(test)]
mod tests {
    use super::*;

    const ANSWER: &str = "Hello! I hope you are enjoying your trip to Ral'Malgor. Don't forget to pick up some souvenirs for the family while you are there. Perhaps send mom a postcard as well? Also make sure to take some great photos! See you soon!! ... by the way the answer to the question is 'codingquest2022' (without the quotes).";

    #[test]
    fn input_doit()
    {
        let input = include_str!("../input.txt");
        assert_eq!(doit(input), ANSWER.to_string());
    }
}

fn main()
{
    use std::time::Instant;

    let input = include_str!("../input.txt");

    let t = Instant::now();
    let result = doit(input);
    println!("{} ({:?})", result, t.elapsed());
}

fn doit(input: &str) -> i32
{
    let winners = ["12", "48", "30", "95", "15", "55", "97"];
    input.lines()
        .fold(0, |acc, line| {
            let count = line.split(' ')
                .filter(|s| winners.contains(s))
                .count() as u32;
            acc + if count > 2 { 10_i32.pow(count - 3) } else { 0 }
        })
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_doit()
    {
        let input = include_str!("../input.txt");
        assert_eq!(doit(input), 56);
    }

    #[test]
    fn example_doit()
    {
        let input = include_str!("../example.txt");
        assert_eq!(doit(input), 110);
    }
}

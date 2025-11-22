fn main()
{
    use std::time::Instant;

    let input = include_str!("../input.txt");

    let t = Instant::now();
    let result = doit(input);
    println!("{} ({:?})", result, t.elapsed());
}

fn doit(input: &str) -> u64
{
    use std::collections::HashMap;

    type Items<'a> = HashMap<&'a str, u64>;

    input.lines()
        .fold(Items::new(), |mut m, line| {
            let mut iter = line.split_ascii_whitespace().skip(1);
            let count = iter.next().unwrap().parse::<u64>().unwrap();
            let category = iter.next().unwrap();
            *m.entry(category).or_default() += count;
            m
        })
        .values()
        .map(|v| *v % 100)
        .product()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_doit()
    {
        let input = include_str!("../input.txt");
        assert_eq!(doit(input), 13327755200);
    }

    #[test]
    fn example_doit()
    {
        let input = include_str!("../example.txt");
        assert_eq!(doit(input), 187733700);
    }
}

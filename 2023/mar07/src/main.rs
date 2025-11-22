fn main()
{
    use std::time::Instant;

    let input = include_str!("../input.txt");

    let t = Instant::now();
    let result = doit(input);
    println!("{} ({:?})", result, t.elapsed());
}

fn doit(input: &str) -> u32
{
    use rounded_div::RoundedDiv;

    let mut count = 0;
    let total = input.lines()
        .map(|line| line.parse::<u16>().unwrap())
        .filter(|n| n.count_ones() % 2 == 0)
        .map(|n| { count += 1; (n & 0x7FFF) as u32 })
        .sum::<u32>();
    
    total.rounded_div(count)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_doit()
    {
        let input = include_str!("../input.txt");
        assert_eq!(doit(input), 297);
    }

    #[test]
    fn example_doit()
    {
        let input = include_str!("../example.txt");
        assert_eq!(doit(input), 17837);
    }
}

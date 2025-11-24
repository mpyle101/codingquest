fn main()
{
    use std::time::Instant;

    let input = include_str!("../input.txt");

    let t = Instant::now();
    let result = doit(input);
    println!("{} ({:?})", result, t.elapsed());
}

fn doit(input: &str) -> String
{
    let mut col = 0;
    let mut char = '.';
    input.split_ascii_whitespace()
        .for_each(|s| {
            let n = s.parse::<u32>().unwrap();
            for _ in 0..n {
                print!("{char}");
                col += 1;
                if col == 100 {
                    println!();
                    col = 0;
                }
            }
            char = if char == '.' { '#' } else { '.' }
        });
    
    "4951".into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_doit()
    {
        let input = include_str!("../input.txt");
        assert_eq!(doit(input), "4951".to_string());
    }
}

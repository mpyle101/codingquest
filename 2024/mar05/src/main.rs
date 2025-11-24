use std::num::ParseIntError;

fn main()
{
    use std::time::Instant;

    let input = include_str!("../input.txt");

    let t = Instant::now();
    let result = doit(input);
    println!("{:?} ({:?})", result, t.elapsed());
}

const SHIP: std::ops::Range<u32> = 0xC0A80000..0xC0A8FFFF;
const WIFI: std::ops::Range<u32> = 0x0A000000..0xA00000FF;

fn doit(input: &str) -> (u32, u32)
{
    input.lines()
        .fold((0, 0), |acc, line| {
            let b = decode(line).unwrap();
            let len = (b[2] as u32) << 8 | b[3] as u32;
            let src = u32::from_be_bytes([b[12], b[13], b[14], b[15]]);
            let dst = u32::from_be_bytes([b[16], b[17], b[18], b[19]]);

            if SHIP.contains(&src) || SHIP.contains(&dst) {
                (acc.0 + len, acc.1)
            } else if WIFI.contains(&src) || WIFI.contains(&dst) {
                (acc.0, acc.1 + len)
            } else {
                acc
            }
        })
}

fn decode(s: &str) -> Result<Vec<u8>, ParseIntError>
{
    (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i+2], 16))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_doit()
    {
        let input = include_str!("../input.txt");
        assert_eq!(doit(input), (258956, 256237));
    }

    #[test]
    fn example_doit()
    {
        let input = include_str!("../example.txt");
        assert_eq!(doit(input), (868, 1625));
    }
}

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
    let mut msgs = input.lines()
        .map(|line|
            (0..line.len()).step_by(2)
                .map(|i| u8::from_str_radix(&line[i..i + 2], 16).unwrap())
                .collect::<Vec<_>>()
        ) 
        .filter(|bytes| bytes[0] == 0x55 && bytes[1] == 0x55)
        .map(|bytes| {
            let sender = u32::from_be_bytes(bytes[2..6].try_into().unwrap());
            let seqno  = bytes[6];
            let cksum  = bytes[7];
            let msg: [u8;24] = bytes[8..].try_into().unwrap();
            (sender, seqno, cksum, msg)
        })
        .filter(|t| {
            let v = t.3.iter().map(|n| *n as u32).sum::<u32>() % 256;
            v == t.2 as u32
        })
        .map(|t| (t.1, t.3))
        .collect::<Vec<_>>();
    msgs.sort();
    let message = msgs.iter()
        .map(|(_, msg)| msg.iter().map(|b| *b as char).collect::<String>())
        .collect::<Vec<_>>()
        .join("");

    message.trim_end().to_string()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_doit()
    {
        let input = include_str!("../input.txt");
        assert_eq!(doit(input), "This is the SY Titanic II. We have struck an asteroid. Engines non responsive. Please help. Coordinates 242.03 by 182.24 by 92.58. Message ends.".to_string());
    }

    #[test]
    fn example_doit()
    {
        let input = include_str!("../example.txt");
        assert_eq!(doit(input), "This is a test. This is a test. Thankyou.".to_string());
    }
}

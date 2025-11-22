fn main()
{
    use std::time::Instant;

    let input = include_bytes!("../input.png");

    let t = Instant::now();
    let result = doit(input);
    println!("{} ({:?})", result, t.elapsed());
}

fn doit(input: &[u8]) -> String
{
    use lodepng::Image;
    use lodepng::ColorType::RGB;

    let image = lodepng::decode_memory(input, RGB, 8).unwrap();
    if let Image::RGB(bitmap) = image {
        let message = bitmap.buffer.chunks(8)
            .map(|chunk| {
                let mut byte = 0;
                byte = setbit(byte, 7, chunk[0].r & 1);
                byte = setbit(byte, 6, chunk[1].r & 1);
                byte = setbit(byte, 5, chunk[2].r & 1);
                byte = setbit(byte, 4, chunk[3].r & 1);
                byte = setbit(byte, 3, chunk[4].r & 1);
                byte = setbit(byte, 2, chunk[5].r & 1);
                byte = setbit(byte, 1, chunk[6].r & 1);
                byte = setbit(byte, 0, chunk[7].r & 1);
                byte as char
            })
            .collect::<String>();

        return message.rsplit_once(' ')
            .unwrap().1.into()
    }

    panic!()
}

pub fn setbit(x: u8, i: u8, b: u8) -> u8
{
    let mask = !(1 << i);
    let flag = b << i;
    x & mask | flag
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_doit()
    {
        let input = include_bytes!("../input.png");
        assert_eq!(doit(input), "cake.".to_string());
    }
}

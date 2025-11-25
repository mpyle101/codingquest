use std::collections::HashMap;

type FileSystem<'a> = HashMap<&'a str, Vec<(&'a str, &'a str)>>;

fn main()
{
    use std::time::Instant;

    let input = include_str!("../input.txt");

    let t = Instant::now();
    let result = doit(input);
    println!("{:?} ({:?})", result, t.elapsed());
}

fn doit(input: &str) -> u64
{
    let fs = input.split("Folder: ")
        .filter(|s| !s.is_empty())
        .map(|s1| {
            let mut iter = s1.lines();
            let folder = iter.next().unwrap();
            let contents = iter.map(|s2| s2[3..].split_once(' ').unwrap()).collect::<Vec<_>>();

            (folder, contents)
        })
        .collect::<FileSystem>();

    calc_deleted(&fs, "0", false)
}

fn calc_deleted(fs: &FileSystem, folder: &str, delete: bool) -> u64
{
    let contents = fs.get(folder).unwrap();

    if delete {
        contents.iter()
            .map(|(_, size)| {
                if size.starts_with("[") {
                    let (_, s) = size.split_once(' ').unwrap();
                    calc_deleted(fs, s.trim_end_matches(']'), delete)
                } else {
                    size.parse::<u64>().unwrap()
                }
            })
            .sum()
    } else {
        contents.iter()
            .map(|(name, size)| {
                let delete = name.contains("delete") || name.contains("temporary");
                if size.starts_with("[") {
                    let (_, s) = size.split_once(' ').unwrap();
                    calc_deleted(fs, s.trim_end_matches(']'), delete)
                } else if delete {
                    size.parse::<u64>().unwrap()
                } else {
                    0
                }
            })
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_doit()
    {
        let input = include_str!("../input.txt");
        assert_eq!(doit(input), 349035592144);
    }

    #[test]
    fn example_doit()
    {
        let input = include_str!("../example.txt");
        assert_eq!(doit(input), 103879262);
    }
}

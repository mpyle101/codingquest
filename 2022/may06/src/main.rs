fn main()
{
    let input = include_str!("../input.txt");
    println!("{}", doit(input));
}

fn doit(input: &str) -> String
{
    let mut items: Vec<_> = input.lines()
        .map(|s| {
            let (record, hash) = s.rsplit_once('|').unwrap();

            let mut iter = record.split('|');
            let desc  = iter.next().unwrap();
            let mined = iter.next().unwrap().parse::<u32>().unwrap();
            let prev  = iter.next().unwrap().to_string();

            Item { desc, hash, prev, record, mined }
        })
        .collect();

    let mut forgery = 0;
    for (i, item) in items.iter().enumerate().skip(5) {
        if item.record_hash() != item.hash {
            forgery = i;
            break;
        }
    }

    for i in forgery+1..items.len() {
        items[i].prev = mine(items[i-1].clone());
    }

    mine(items.last().unwrap().clone())
}

#[derive(Clone)]
struct Item<'a> {
    desc: &'a str,
    hash: &'a str,
    prev: String,
    mined: u32,
    record: &'a str,
}
impl<'a> Item<'a> {
    fn hash(&self) -> String
    {
        use sha256::digest;
        digest(format!("{}|{}|{}", self.desc, self.mined, self.prev))
    }

    fn record_hash(&self) -> String
    {
        use sha256::digest;
        digest(self.record)
    }
}

fn mine(item: Item) -> String
{
    let mut hash: String;

    let mut item = item;
    item.mined = 0;
    loop {
        hash = item.hash();
        if hash.starts_with("000000") {
            break;
        }
        item.mined += 1;
    }

    hash
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_doit()
    {
        let input = include_str!("../input.txt");
        assert_eq!(doit(input), "000000b60719f04f18d3ae69d12449e48d25dbb1d2e0514ff4d8decede19f728");
    }
}

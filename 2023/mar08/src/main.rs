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
    input.lines()
        .fold([0, 0, 0], |mut wins, line| {
            let mut moves = [0usize;9];
            line.as_bytes()
                .chunks(2)
                .enumerate()
                .for_each(|(i, ch)| moves[i] = (ch[0] - b'0') as usize);
            wins[play(moves)] += 1;
            wins
        })
        .iter()
        .product()
}

fn play(moves: [usize;9]) -> usize
{
    let mut board = [2u8;10];

    let mut player = 0;
    moves.into_iter()
        .take(5)
        .for_each(|i| { 
            board[i] = player;
            player = (player + 1) % 2;
        });

    for i in moves.into_iter().skip(5) {
        if let Some(player) = win(&board) {
            return player
        }
        board[i] = player;
        player = (player + 1) % 2;
    }

    if let Some(player) = win(&board) {
        return player
    }

    2
}

const WINS: [[usize;3]; 8] = [
    [1, 2, 3],
    [1, 5, 9],
    [1, 4, 7],
    [2, 5, 8],
    [3, 6, 9],
    [3, 5, 7],
    [4, 5, 6],
    [7, 8, 9],
];

fn win(board: &[u8;10]) -> Option<usize>
{
    for tiles in WINS {
        let val = [board[tiles[0]], board[tiles[1]], board[tiles[2]]];
        let win = val[0] != 2 && val[1] == val[0] && val[2] == val[1];
        if win {
            return Some(val[0] as usize)
        }
    }

    None
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_doit()
    {
        let input = include_str!("../input.txt");
        assert_eq!(doit(input), 20938290);
    }
}

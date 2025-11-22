fn main()
{
    let input = include_str!("../input.txt");
    doit(input);
}

fn doit(input: &str)
{
    let mut vm = VM::new(input);
    vm.run();

    // 7745743850156157
}

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug)]
enum OpCode {
    ADV(usize, i64),
    ADR(usize, usize),
    MOD(usize, i64),
    DIV(usize, i64),
    MVV(usize, i64),
    MVR(usize, usize),
    JMP(i8),
    JIF(i8),
    EQV(usize, i64),
    EQR(usize, usize),
    GEV(usize, i64),
    GER(usize, usize),
    OTV(i64),
    OTR(usize),
    END,
}

struct VM {
    program: Vec<OpCode>,
}
impl VM {
    fn new(input: &str) -> VM
    {
        let program = read(input);
        VM { program }
    }

    fn run(&mut self)
    {
        use OpCode::*;

        let mut ip = 0;
        let mut reg = false;
        let mut vars = [0i64;12];
        let mut done = false;

        while !done {
            //println!("{ip} {:?} {:?}", vars, self.program[ip as usize]);
            ip = match self.program[ip as usize] {
                ADV(i, n) => { vars[i] += n; ip + 1 },
                ADR(a, b) => { vars[a] += vars[b]; ip + 1 },
                MOD(i, n) => { vars[i] %= n; ip + 1 },
                DIV(i, n) => { vars[i] /= n; ip + 1 },
                MVV(i, n) => { vars[i] = n; ip + 1 },
                MVR(a, b) => { vars[a] = vars[b]; ip + 1 },
                JMP(n)    => ip + n,
                JIF(n)    => if reg { ip + n } else { ip + 1 },
                EQV(i, n) => { reg = vars[i] == n; ip + 1 }
                EQR(a, b) => { reg = vars[a] == vars[b]; ip + 1 }
                GEV(i, n) => { reg = vars[i] >= n; ip + 1 }
                GER(a, b) => { reg = vars[a] >= vars[b]; ip + 1 }
                OTV(n)    => { println!("{n}"); ip + 1 }
                OTR(i)    => { println!("{}", vars[i]); ip + 1 }
                END       => { done = true; ip }
            };
        }
    }
}

fn read(input: &str) -> Vec<OpCode>
{
    use OpCode::*;

    input.lines()
        .map(|line| {
            let mut iter = line.split_ascii_whitespace();
            match iter.next().unwrap() {
                "ADD" => {
                    let i = (iter.next().unwrap().bytes().next().unwrap() - b'A') as usize;
                    let s = iter.next().unwrap();
                    match s.parse::<i64>() {
                        Ok(n)  => ADV(i, n),
                        Err(_) => {
                            let b = (s.bytes().next().unwrap() - b'A') as usize;
                            ADR(i, b)
                        }
                    }
                }
                "MOD" => {
                    let i = (iter.next().unwrap().bytes().next().unwrap() - b'A') as usize;
                    let n = iter.next().unwrap().parse::<i64>().unwrap();
                    MOD(i, n)
                }
                "DIV" => {
                    let i = (iter.next().unwrap().bytes().next().unwrap() - b'A') as usize;
                    let n = iter.next().unwrap().parse::<i64>().unwrap();
                    DIV(i, n)
                }
                "MOV" => {
                    let i = (iter.next().unwrap().bytes().next().unwrap() - b'A') as usize;
                    let s = iter.next().unwrap();
                    match s.parse::<i64>() {
                        Ok(n)  => MVV(i, n),
                        Err(_) => {
                            let b = (s.bytes().next().unwrap() - b'A') as usize;
                            MVR(i, b)
                        }
                    }
                }
                "JMP" => {
                    let i = iter.next().unwrap().parse::<i8>().unwrap();
                    JMP(i)
                }
                "JIF" => {
                    let i = iter.next().unwrap().parse::<i8>().unwrap();
                    JIF(i)
                }
                "CEQ" => {
                    let i = (iter.next().unwrap().bytes().next().unwrap() - b'A') as usize;
                    let s = iter.next().unwrap();
                    match s.parse::<i64>() {
                        Ok(n)  => EQV(i, n),
                        Err(_) => {
                            let b = (s.bytes().next().unwrap() - b'A') as usize;
                            EQR(i, b)
                        }
                    }
                }
                "CGE" => {
                    let i = (iter.next().unwrap().bytes().next().unwrap() - b'A') as usize;
                    let s = iter.next().unwrap();
                    match s.parse::<i64>() {
                        Ok(n)  => GEV(i, n),
                        Err(_) => {
                            let b = (s.bytes().next().unwrap() - b'A') as usize;
                            GER(i, b)
                        }
                    }
                }
                "OUT" => {
                    let s = iter.next().unwrap();
                    match s.parse::<i64>() {
                        Ok(n)  => OTV(n),
                        Err(_) => {
                            let i = (s.bytes().next().unwrap() - b'A') as usize;
                            OTR(i)
                        }
                    }
                }
                "END" => END,
               _ => panic!("Unknown opcode")
            }
        })
        .collect()
}

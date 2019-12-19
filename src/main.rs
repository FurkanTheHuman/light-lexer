use std::fmt;
use std::io;
use std::iter::Peekable;

fn simple_input() -> String {
    let mut i = String::new();
    io::stdin().read_line(&mut i).expect("err");
    i
}
enum Token {
    Number(i32),
    Name(String),
}

struct Position {
    start: u32,
    end: u32,
    line: u32,
}

struct TokenData {
    token: Token,
    position: Position,
}
fn consume_number<T: Iterator<Item = char>>(
    c: char,
    iter: &mut Peekable<T>,
    pos: &mut Position,
) -> i32 {
    let mut number = c
        .to_string()
        .parse::<i32>()
        .expect("The caller should have passed a digit.");
    while let Some(Ok(digit)) = iter.peek().map(|c| c.to_string().parse::<i32>()) {
        number = number * 10 + digit;
        iter.next();
        pos.end += 1;
    }
    number
}

fn consume_name<T: Iterator<Item = char>>(iter: &mut Peekable<T>, pos: &mut Position) -> String {
    let mut name = String::new();
    while let Some(c) = iter.next() {
        if c == ' ' || c == '\n' {
            break;
        }
        name.push(c);
        pos.end += 1;
    }
    pos.start = pos.end;
    name
}

fn tokenize(code: String) -> Vec<Token> {
    let mut current_position: Position = Position {
        start: 0,
        end: 0,
        line: 0,
    };
    let mut token_vec = Vec::new();
    //Mostly supports everything I want but not exactly Unicode
    let mut it = code.chars().peekable();

    while let Some(&c) = it.peek() {
        match c {
            '0'..='9' => {
                it.next();
                current_position.end += 1;
                let t: Token = Token::Number(consume_number(c, &mut it, &mut current_position));
                token_vec.push(t);
            }
            'a'..='z' => {
                let t: Token = Token::Name(consume_name(&mut it, &mut current_position));
                token_vec.push(t);
            }
            ' ' => {
                it.next();
            }
            '\n' => break,
            _ => {
                println!("ERRR {}", c);
                break;
            }
        }
    }

    token_vec
}

impl fmt::Debug for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &*self {
            Token::Name(n) => write!(f, "[Name] => \"{}\"", n),
            Token::Number(n) => write!(f, "[Number] => \"{}\"", n),
        }
    }
}

impl fmt::Debug for TokenData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(
            f,
            "[ (token: {:?}), line: {} range({}, {})]",
            self.token, self.position.line, self.position.start, self.position.end
        )
    }
}

fn main() {
    loop {
        let i = simple_input();
        let list = tokenize(i);
        println!("{:?}", list);
    }
}

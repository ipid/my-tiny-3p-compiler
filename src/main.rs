use std::collections::{VecDeque, HashMap};
use std::convert::TryFrom;
use crate::Ast::{BinOp, UnOp};

#[derive(Debug)]
enum Ast {
    BinOp(String, Box<Ast>, Box<Ast>),
    UnOp(String, i32),
}

impl Ast {
    fn is_imm(&self) -> bool {
        match self {
            Ast::UnOp(tag, _) => tag == "imm",
            _ => false,
        }
    }

    fn is_arg(&self) -> bool {
        match self {
            Ast::UnOp(tag, _) => tag == "arg",
            _ => false,
        }
    }

    fn op(&self) -> &String {
        match self {
            Ast::UnOp(s, _) => s,
            Ast::BinOp(s, ..) => s,
        }
    }
}

fn is_number_token(x: &str) -> bool {
    let first_char = x.chars().nth(0).unwrap();
    '0' <= first_char && first_char <= '9'
}

struct Pass1 {
    tokens: VecDeque<String>,
    symbols: HashMap<String, usize>,
}

impl Pass1 {
    fn new(program: &str) -> Pass1 {
        Pass1 {
            tokens: Pass1::tokenize(program),
            symbols: HashMap::new(),
        }
    }

    fn tokenize(program: &str) -> VecDeque<String> {
        // Codewar 自带的代码

        let mut tokens: VecDeque<String> = VecDeque::new();
        let mut iter = program.chars().peekable();

        loop {
            match iter.peek() {
                Some(&c) => match c {
                    'a'..='z' | 'A'..='Z' => {
                        let mut tmp = String::new();
                        while iter.peek().is_some() && iter.peek().unwrap().is_alphabetic() {
                            tmp.push(iter.next().unwrap());
                        }
                        tokens.push_back(tmp);
                    }
                    '0'..='9' => {
                        let mut tmp = String::new();
                        while iter.peek().is_some() && iter.peek().unwrap().is_numeric() {
                            tmp.push(iter.next().unwrap());
                        }
                        tokens.push_back(tmp);
                    }
                    ' ' => { iter.next(); }
                    _ => {
                        tokens.push_back(iter.next().unwrap().to_string());
                    }
                },
                None => break
            }
        }
        tokens.push_back("$".to_string());

        tokens
    }

    fn parse(mut self) -> Ast {
        let mut last_unused = 0usize;

        // TODO: 这里是不是该用迭代器？
        self.tokens.pop_front().unwrap();
        while !(self.tokens.front().unwrap() == "]") {
            self.symbols.insert(self.tokens.pop_front().unwrap(), last_unused);
            last_unused += 1;
        }

        self.tokens.pop_front().unwrap();

        self.expression()
    }

    fn expression(&mut self) -> Ast {
        // TODO: 这里我不想用 mut 的话该怎么写？
        let mut result = self.term();

        while self.tokens.front().unwrap() == "+" || self.tokens.front().unwrap() == "-" {
            let op = self.tokens.pop_front().unwrap();
            let right = self.term();
            result = BinOp(op, Box::new(result), Box::new(right));
        }

        result
    }

    fn term(&mut self) -> Ast {
        // TODO: 这里我不想用 mut 的话该怎么写？
        let mut result = self.factor();

        while self.tokens.front().unwrap() == "*" || self.tokens.front().unwrap() == "/" {
            let op = self.tokens.pop_front().unwrap();
            let right = self.factor();
            result = BinOp(op, Box::new(result), Box::new(right));
        }

        result
    }

    fn factor(&mut self) -> Ast {
        let first = self.tokens.pop_front().unwrap();

        // TODO: 这里感觉写得乱七八糟的怎么办
        if first == "(" {
            let result = self.expression();
            self.tokens.pop_front().unwrap();

            result
        } else if is_number_token(&first) {
            let num: i32 = first.parse().unwrap();

            Ast::UnOp("imm".to_string(), num)
        } else {
            let index = *self.symbols.get(&first).unwrap();

            UnOp("arg".to_string(), i32::try_from(index).unwrap())
        }
    }
}

struct Compiler {}

impl Compiler {
    fn new() -> Compiler {
        Compiler {}
    }

    fn pass1(&self, program: &str) -> Ast {
        Pass1::new(program).parse()
    }
}

fn main() {
    let code = "[ x y z ] ( 2*3*x + 5*y - 3*z ) / (1 + 3 + 2*2)";
    let compiler = Compiler::new();

    // 打印语法树
    println!("{:#?}", compiler.pass1(code));
}
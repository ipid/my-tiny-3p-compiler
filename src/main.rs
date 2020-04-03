enum Ast {
    // your code
}

struct Compiler {
    // your code
}

impl Compiler {
    fn new() -> Compiler {
        Compiler { }
    }

    fn tokenize<'a>(&self, program : &'a str) -> Vec<String> {
        let mut tokens : Vec<String> = vec![];

        let mut iter = program.chars().peekable();
        loop {
            match iter.peek() {
                Some(&c) => match c {
                    'a'...'z'|'A'...'Z' => {
                        let mut tmp = String::new();
                        while iter.peek().is_some() && iter.peek().unwrap().is_alphabetic() {
                            tmp.push(iter.next().unwrap());
                        }
                        tokens.push(tmp);
                    },
                    '0'...'9' => {
                        let mut tmp = String::new();
                        while iter.peek().is_some() && iter.peek().unwrap().is_numeric() {
                            tmp.push(iter.next().unwrap());
                        }
                        tokens.push(tmp);
                    },
                    ' ' => { iter.next(); },
                    _ => {
                        tokens.push(iter.next().unwrap().to_string());
                    },
                },
                None => break
            }
        }

        tokens
    }

    fn compile(&mut self, program : &str) -> Vec<String> {
        let ast = self.pass1(program);
        let ast = self.pass2(&ast);
        self.pass3(&ast)
    }

    fn pass1(&mut self, program : &str) -> Ast {
        let tokens = self.tokenize(program);
        let mut iter = tokens.iter().peekable();
        // your code
    }

    fn pass2(&mut self, ast : &Ast) -> Ast {
        // your code
    }

    fn pass3(&mut self, ast : &Ast) -> Vec<String> {
        // your code
    }
}

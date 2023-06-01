use anyhow::Result;

#[derive(Debug, PartialEq)]
pub enum Token {
    IDENT(String),
    INT(String),
//    ILLEGAL,
    EOF,
    ASSIGN,
    PLUS,
    COMMA,
    SEMICOLON,
    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,
    FUNCTION,
    LET,
 //   IF,
}

pub struct Lexer {
    position: usize,
    read_position: usize,
    ch: u8,
    input: Vec<u8>,
}

impl Lexer {
    fn new(input: String) -> Lexer {
        let mut lex = Lexer {
            position: 0,
            read_position: 0,
            ch: 0,
            input: input.into_bytes(),
        };

        lex.read_char();
        return lex;
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = 0
        } else {
            self.ch = self.input[self.read_position]
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    fn next_token(&mut self) -> Result<Token> {
        self.skip_whitespace();     
        let tok = match self.ch {
            b'{' => Token::LBRACE,
            b'}' => Token::RBRACE,
            b'(' => Token::LPAREN,
            b')' => Token::RPAREN,
            b',' => Token::COMMA,
            b';' => Token::SEMICOLON,
            b'+' => Token::PLUS,
            b'=' => Token::ASSIGN,
            b'a'..=b'z' | b'A'..=b'Z' | b'_' => {
                let ident = self.read_ident();
                return Ok(match ident.as_str() {
                    "fn" => Token::FUNCTION,
                    "let" => Token::LET,
                    _ => Token::IDENT(ident),
                });
            },
            b'0'..=b'9' => return Ok(Token::INT(self.read_int())),
            0 => Token::EOF,
            _ => todo!("needs implementation"),
        };

        self.read_char();
        return Ok(tok);
    }

    fn skip_whitespace(&mut self) {
        while self.ch.is_ascii_whitespace() {
            self.read_char();
        }
    }

    fn read_ident(&mut self) -> String {
        let pos = self.position;
        while self.ch.is_ascii_alphabetic() || self.ch == b'_' {
            self.read_char();
        }
        return String::from_utf8_lossy(&self.input[pos..self.position]).to_string();
    }
    
    fn read_int(&mut self) -> String {
        let pos = self.position;
        while self.ch.is_ascii_digit() {
            self.read_char();
        }

        return String::from_utf8_lossy(&self.input[pos..self.position]).to_string();
    }
}

#[cfg(test)]
mod test {
    use anyhow::Result;
    use super::{Lexer, Token};

    #[test]
    fn test_next_token1() -> Result<()> {
        let input = "(){},;+=";
        let mut lexer = Lexer::new(input.into());
        let tokens = vec![
            Token::LPAREN,
            Token::RPAREN,
            Token::LBRACE,
            Token::RBRACE,
            Token::COMMA,
            Token::SEMICOLON,
            Token::PLUS,
            Token::ASSIGN,
        ];
        for token in tokens {
            let nex_token = lexer.next_token()?;
            println!("expected: {:?}, recieved: {:?}", token, nex_token);
            assert_eq!(token, nex_token);
        }
        return Ok(());
    }

    #[test]
    fn test_next_token2() -> Result<()> {
        let input = r#"let five = 5;
        let ten = 10;
        let add = fn(x, y) {
            x + y;
        };
        let result = add(five, ten);"#;
        let mut lexer = Lexer::new(input.into());
        let tokens = vec![
            Token::LET,
            Token::IDENT(String::from("five")),
            Token::ASSIGN,
            Token::INT(String::from("5")),
            Token::SEMICOLON,
            Token::LET,
            Token::IDENT(String::from("ten")),
            Token::ASSIGN,
            Token::INT(String::from("10")),
            Token::SEMICOLON,
            Token::LET,
            Token::IDENT(String::from("add")),
            Token::ASSIGN,
            Token::FUNCTION,
            Token::LPAREN,
            Token::IDENT(String::from("x")),
            Token::COMMA,
            Token::IDENT(String::from("y")),
            Token::RPAREN,
            Token::LBRACE,
            Token::IDENT(String::from("x")),
            Token::PLUS,
            Token::IDENT(String::from("y")),
            Token::SEMICOLON,
            Token::RBRACE,
            Token::SEMICOLON,
            Token::LET,
            Token::IDENT(String::from("result")),
            Token::ASSIGN,
            Token::IDENT(String::from("add")),
            Token::LPAREN,
            Token::IDENT(String::from("five")),
            Token::COMMA,
            Token::IDENT(String::from("ten")),
            Token::RPAREN,
            Token::SEMICOLON,
        ];

        for token in tokens {
            let next_token = lexer.next_token()?;
            println!("expected: {:?}, recieved: {:?}", token, next_token);
            assert_eq!(token, next_token);
        }
        return Ok(());
    }
}

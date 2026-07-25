use std::fs::File;
use std::io::{BufRead, BufReader};

use crate::err::LexerError::*;
use crate::{err::LexerError, token::Token};

// lexer takes a file

pub struct Lexer {
    reader: BufReader<File>,
    chars: Vec<char>,
    pos: usize,
    eof: bool,
}

impl Lexer {
    pub fn new(input: File) -> Self {
        let mut lexer = Self {
            reader: BufReader::new(input), // reader for file
            chars: Vec::new(),             // stores current line
            pos: 0,                        // index in line
            eof: false,                    // eof
        };

        // Read first line
        lexer.next_line();
        lexer
    }

    fn next_line(&mut self) -> bool {
        let mut buf = String::new();

        let size = self
            .reader
            .read_line(&mut buf)
            .expect("read_char: Error while reading");

        if size == 0 {
            self.eof = true;
            self.chars = Vec::new();
            self.pos = 0;
            return false;
        }

        Self::trim_line(&mut buf);

        self.chars = buf.chars().collect();
        self.pos = 0;
        true
    }

    pub fn advance(&mut self) -> bool {
        self.pos += 1;

        // If we're at the end of current line, try to read next
        while self.pos >= self.chars.len() && !self.eof {
            if !self.next_line() {
                return false;
            }
        }

        true
    }

    fn trim_line(str: &mut String) {
        let index = str.find("//");
        if let Some(i) = index {
            let res: String = str.chars().take(i).collect();
            *str = res.trim_ascii().to_string();
        } else {
            *str = str.trim_ascii().to_string()
        }
    }

    fn current_char(&self) -> Option<char> {
        if self.eof || self.pos >= self.chars.len() {
            None
        } else {
            Some(self.chars[self.pos])
        }
    }

    fn peek_char(&self) -> Option<char> {
        if self.eof || self.pos + 1 >= self.chars.len() {
            None
        } else {
            Some(self.chars[self.pos + 1])
        }
    }

    pub fn next_token(&mut self) -> Result<Token, LexerError> {
        // Skip whitespace
        while let Some(c) = self.current_char() {
            if !c.is_ascii_whitespace() {
                break;
            }
            self.advance();
        }

        // Check for EOF
        let first_char = match self.current_char() {
            Some(c) => c,
            None => return Err(ReadAfterEnd()),
        };

        // Two-character tokens
        if first_char == '-' {
            if let Some(next) = self.peek_char() {
                if next == '>' {
                    self.advance(); // consume '-'
                    self.advance(); // consume '>'
                    return Ok(Token::Arrow());
                }
            }
        }

        // Single-character tokens
        match first_char {
            '+' => {
                self.advance();
                return Ok(Token::Plus());
            }
            '-' => {
                self.advance();
                return Ok(Token::Minus());
            }
            '&' => {
                self.advance();
                return Ok(Token::And());
            }
            '=' => {
                self.advance();
                return Ok(Token::Eq());
            }
            '|' => {
                self.advance();
                return Ok(Token::Or());
            }
            '^' => {
                self.advance();
                return Ok(Token::Xor());
            }
            _ => {}
        }

        // Multi-character tokens
        let mut token = String::new();

        if first_char.is_ascii_alphabetic() {
            // Identifier or keyword: letters, digits, underscores
            while let Some(c) = self.current_char() {
                if c.is_ascii_alphanumeric() || c == '_' {
                    token.push(c);
                    self.advance();
                } else {
                    break;
                }
            }

            // Check for keywords
            Ok(match token.as_str() {
                "fun" => Token::Fun(),
                _ => Token::Identifier(token),
            })
        } else if first_char.is_ascii_digit() {
            // Number: digits only
            while let Some(c) = self.current_char() {
                if c.is_ascii_digit() {
                    token.push(c);
                    self.advance();
                } else {
                    break;
                }
            }
            Ok(Token::Word(token.parse().unwrap()))
        } else {
            // Unknown - collect until whitespace
            while let Some(c) = self.current_char() {
                if c.is_ascii_whitespace() {
                    break;
                }
                token.push(c);
                self.advance();
            }
            Ok(Token::Identifier(token))
        }
    }

    pub fn eof(&self) -> bool {
        self.eof
    }
}

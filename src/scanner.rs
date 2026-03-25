use crate::token::TokenType;
use crate::utils::is_aplha_numeric;

#[derive(Debug)]
pub struct Scanner {
    tokens: Vec<TokenType>,
}

impl Scanner {
    pub fn new() -> Self {
        Self { tokens: Vec::new() }
    }

    pub fn scan_tokens(&mut self, source: &str) {
        let mut line = 1;
        let mut invalid_source_code = false;
        let source_list: Vec<char> = source.chars().collect();
        let mut i = 0;
        while i < source_list.len() {
            match source_list[i] {
                '(' => self.tokens.push(TokenType::LeftParen(line)),
                ')' => self.tokens.push(TokenType::RightParen(line)),
                '{' => self.tokens.push(TokenType::LeftBrace(line)),
                '}' => self.tokens.push(TokenType::RightBrace(line)),
                ',' => self.tokens.push(TokenType::Comma(line)),
                '.' => self.tokens.push(TokenType::Dot(line)),
                '-' => self.tokens.push(TokenType::Minus(line)),
                '+' => self.tokens.push(TokenType::Plus(line)),
                ';' => self.tokens.push(TokenType::SemiColon(line)),
                '/' => self.tokens.push(TokenType::Slash(line)),
                '*' => self.tokens.push(TokenType::Star(line)),
                '!' => {
                    if i + 1 < source_list.len() && source_list[i + 1] == '=' {
                        self.tokens.push(TokenType::BangEqual(line));
                    } else {
                        self.tokens.push(TokenType::Bang(line));
                    }
                }
                '=' => {
                    if i + 1 < source_list.len() && source_list[i + 1] == '=' {
                        self.tokens.push(TokenType::EqualEqual(line));
                    } else {
                        self.tokens.push(TokenType::Equal(line));
                    }
                }
                '>' => {
                    if i + 1 < source_list.len() && source_list[i + 1] == '=' {
                        self.tokens.push(TokenType::GreaterEqual(line));
                    } else {
                        self.tokens.push(TokenType::Greater(line));
                    }
                }
                '<' => {
                    if i + 1 < source_list.len() && source_list[i + 1] == '=' {
                        self.tokens.push(TokenType::LessEqual(line));
                    } else {
                        self.tokens.push(TokenType::Less(line));
                    }
                }
                '"' | '\'' => {
                    let mut j = i + 1;
                    while j < source_list.len() && (source_list[j] != '"' && source_list[j] != '\'')
                    {
                        j += 1
                    }
                    if source_list[j] != '"' && source_list[j] != '\'' {
                        invalid_source_code |= true;
                    } else {
                        println!("{:?}", &source_list[i + 1..j]);
                        self.tokens.push(TokenType::String(
                            line,
                            source_list[i + 1..j].iter().collect(),
                        ));
                    }
                    i = j;
                }
                'a'..='z' | 'A'..='Z' => {
                    let keywords = [
                        "and", "if", "else", "or", "print", "return", "true", "false", "while",
                        "for", "var",
                    ];
                    let mut is_keyword_present = false;
                    for keyword in keywords {
                        let j = i + keyword.len();
                        if String::from_iter(source_list[i..j].iter()) == keyword {
                            match keyword {
                                "and" => self.tokens.push(TokenType::And(line)),
                                "if" => self.tokens.push(TokenType::If(line)),
                                "else" => self.tokens.push(TokenType::Else(line)),
                                "or" => self.tokens.push(TokenType::Or(line)),
                                "print" => self.tokens.push(TokenType::Print(line)),
                                "return" => self.tokens.push(TokenType::Return(line)),
                                "true" => self.tokens.push(TokenType::True(line)),
                                "false" => self.tokens.push(TokenType::False(line)),
                                "while" => self.tokens.push(TokenType::While(line)),
                                "for" => self.tokens.push(TokenType::For(line)),
                                "var" => self.tokens.push(TokenType::Var(line)),
                                _ => {
                                    eprintln!("Undefined keyword: {}", keyword);
                                }
                            }
                            is_keyword_present = true;
                            i += keyword.len();
                            break;
                        }
                    }
                    if !is_keyword_present {
                        let mut j = i + 1;
                        while j < source_list.len()
                            && (is_aplha_numeric(source_list[j]) || source_list[j] == '_')
                        {
                            j += 1
                        }
                        if source_list[j] != '"' {
                            invalid_source_code |= true;
                        }
                        println!("{:?}", &source_list[i..j]);
                        self.tokens.push(TokenType::Identifier(
                            line,
                            source_list[i..j].iter().collect(),
                        ));
                        i = j;
                    }
                }
                '0'..'9' => {
                    let mut j = i + 1;
                    while j < source_list.len() && ('0' <= source_list[j] && source_list[j] <= '9')
                    {
                        j += 1;
                    }
                    if j < source_list.len() && source_list[j] == '.' {
                        if j + 1 < source_list.len()
                            && ('0' <= source_list[j + 1] && source_list[j + 1] <= '9')
                        {
                            j += 1;
                            while j < source_list.len()
                                && ('0' <= source_list[j] && source_list[j] <= '9')
                            {
                                j += 1;
                            }
                        } else {
                            invalid_source_code |= true;
                        }
                    }
                    println!("{:?}", &source_list[i..j]);
                    self.tokens.push(TokenType::Number(
                        line,
                        String::from_iter(source_list[i..j].iter()).parse().unwrap(),
                    ));
                    i = j;
                }
                '\n' => line += 1,
                ' ' | '\t' => {}
                '\0' => self.tokens.push(TokenType::EOF(line)),
                _ => {
                    eprintln!("Unexpected character: {}", source_list[i]);
                    invalid_source_code |= true;
                }
            };
            i += 1;
        }
        println!("{:?}", self);
    }
}

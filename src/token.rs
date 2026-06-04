#[derive(Clone, Debug, PartialEq)]
pub enum TokenKind {
    // Single-character tokens
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // One or two character tokens
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals
    Identifier,
    String,
    Number,

    // Keywords
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    Eof,
}

#[derive(Clone, Debug, PartialEq)]
pub enum TokenType {
    // Single-character tokens
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // One or two character tokens
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals
    Identifier(String),
    String(String),
    Number(f64),

    // Keywords
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    Eof,
}

impl TokenType {
    pub fn kind(&self) -> TokenKind {
        match self {
            // Single-character tokens
            TokenType::LeftParen => TokenKind::LeftParen,
            TokenType::RightParen => TokenKind::RightParen,
            TokenType::LeftBrace => TokenKind::LeftBrace,
            TokenType::RightBrace => TokenKind::RightBrace,
            TokenType::Comma => TokenKind::Comma,
            TokenType::Dot => TokenKind::Dot,
            TokenType::Minus => TokenKind::Minus,
            TokenType::Plus => TokenKind::Plus,
            TokenType::Semicolon => TokenKind::Semicolon,
            TokenType::Slash => TokenKind::Slash,
            TokenType::Star => TokenKind::Star,

            // => TokenKind:://,One or two character tokens
            TokenType::Bang => TokenKind::Bang,
            TokenType::BangEqual => TokenKind::BangEqual,
            TokenType::Equal => TokenKind::Equal,
            TokenType::EqualEqual => TokenKind::EqualEqual,
            TokenType::Greater => TokenKind::Greater,
            TokenType::GreaterEqual => TokenKind::GreaterEqual,
            TokenType::Less => TokenKind::Less,
            TokenType::LessEqual => TokenKind::LessEqual,

            // => TokenKind:://,Literals
            TokenType::Identifier(_) => TokenKind::Identifier,
            TokenType::String(_) => TokenKind::String,
            TokenType::Number(_) => TokenKind::Number,

            // => TokenKind:://,Keywords
            TokenType::And => TokenKind::And,
            TokenType::Class => TokenKind::Class,
            TokenType::Else => TokenKind::Else,
            TokenType::False => TokenKind::False,
            TokenType::Fun => TokenKind::Fun,
            TokenType::For => TokenKind::For,
            TokenType::If => TokenKind::If,
            TokenType::Nil => TokenKind::Nil,
            TokenType::Or => TokenKind::Or,
            TokenType::Print => TokenKind::Print,
            TokenType::Return => TokenKind::Return,
            TokenType::Super => TokenKind::Super,
            TokenType::This => TokenKind::This,
            TokenType::True => TokenKind::True,
            TokenType::Var => TokenKind::Var,
            TokenType::While => TokenKind::While,

            TokenType::Eof => TokenKind::Eof,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Token {
    pub r#type: TokenType,
    pub lexeme: String,
    pub line: usize,
}

impl Token {
    pub fn new(r#type: TokenType, lexeme: String, line: usize) -> Self {
        Self {
            r#type,
            lexeme,
            line,
        }
    }
}

pub fn get_keyword(identifier: &str) -> Option<TokenType> {
    match identifier {
        "and" => Some(TokenType::And),
        "class" => Some(TokenType::Class),
        "else" => Some(TokenType::Else),
        "false" => Some(TokenType::False),
        "fun" => Some(TokenType::Fun),
        "for" => Some(TokenType::For),
        "if" => Some(TokenType::If),
        "nil" => Some(TokenType::Nil),
        "or" => Some(TokenType::Or),
        "print" => Some(TokenType::Print),
        "return" => Some(TokenType::Return),
        "super" => Some(TokenType::Super),
        "this" => Some(TokenType::This),
        "true" => Some(TokenType::True),
        "var" => Some(TokenType::Var),
        "while" => Some(TokenType::While),
        _ => None,
    }
}

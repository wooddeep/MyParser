
use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;

#[derive(Clone, Debug, PartialEq)]
pub enum KeyWords {
    Auto,
    Break,
    Case,
    Char,
    Const,
    Continue,
    Default,
    Do,
    Double,
    Else,
    Enum,
    Extern,
    Float,
    For,
    Goto,
    If,
    Inline,
    Int,
    Long,
    Register,
    Restrict,
    Return,
    Short,
    Signed,
    Sizeof,
    Static,
    Struct,
    Switch,
    Typedef,
    Union,
    Unsigned,
    Void,
    Volatile,
    While,
}

impl KeyWords {
    pub fn is_type(&self) -> bool {
        match self {
            // Char | Short | Int | Unsigned | Signed | Long | Double | Float => true,
            Char => true,
            Short => true,
            Int => true,
            Signed => true,
            Unsigned => true,
            Long => true,
            Double => true,
            Float => true,
            _ => false,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Operators {
    Add,
    DoubleAdd,
    Dvision,
    Assign,
    Equal,
    AddEqual,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Brackets {
    LeftParenthesis,
    RightParenthesis,
    LeftSquareBracket,
    RightSquareBracket,
    LeftCurlyBracket,
    RightCurlyBracket,
}

#[derive(Debug, PartialEq)]
pub enum Token {
    Asterisk,
    Bracket(Brackets),
    Comment(String),
    KeyWord(KeyWords),
    LiteralStr(String),
    Number(String),
    Operator(Operators),
    Preprocessor(String),
    Space,
    Semicolon,
    Variable(String),
}

pub fn is_keywords(s: &str) -> bool {
    Token::key_word_index(s).is_some()
}

impl Token {
    pub fn comment(c: &str) -> Token {
        Token::Comment(c.to_owned())
    }

    pub fn variable(v: &str) -> Token {
        Token::Variable(v.to_owned())
    }

    pub fn key_word(k: &str) -> Token {
        const KEY_TOKEN: &'static [KeyWords] = &[
            KeyWords::Auto,
            KeyWords::Break,
            KeyWords::Case,
            KeyWords::Char,
            KeyWords::Const,
            KeyWords::Continue,
            KeyWords::Default,
            KeyWords::Do,
            KeyWords::Double,
            KeyWords::Else,
            KeyWords::Enum,
            KeyWords::Extern,
            KeyWords::Float,
            KeyWords::For,
            KeyWords::Goto,
            KeyWords::If,
            KeyWords::Inline,
            KeyWords::Int,
            KeyWords::Long,
            KeyWords::Register,
            KeyWords::Restrict,
            KeyWords::Return,
            KeyWords::Short,
            KeyWords::Signed,
            KeyWords::Sizeof,
            KeyWords::Static,
            KeyWords::Struct,
            KeyWords::Switch,
            KeyWords::Typedef,
            KeyWords::Union,
            KeyWords::Unsigned,
            KeyWords::Void,
            KeyWords::Volatile,
            KeyWords::While,
        ];
        let index = Token::key_word_index(k).unwrap();

        Token::KeyWord(KEY_TOKEN[index].clone())
    }

    fn key_word_index(s: &str) -> Option<usize> {
        const KEY_WORDS: &'static [&'static str] = &[
            "auto",
            "break",
            "case",
            "char",
            "const",
            "continue",
            "default",
            "do",
            "double",
            "else",
            "enum",
            "extern",
            "float",
            "for",
            "goto",
            "if",
            "inline",
            "int",
            "long",
            "register",
            "restrict",
            "return",
            "short",
            "signed",
            "sizeof",
            "static",
            "struct",
            "switch",
            "typedef",
            "union",
            "unsigned",
            "void",
            "volatile",
            "while",
        ];

        KEY_WORDS.iter().position(|&x| x == s)
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            &Token::Space => write!(f, "space"),
            &Token::Semicolon => write!(f, "semicolon"),
            &Token::Asterisk => write!(f, "asterisk"),
            &Token::LiteralStr(ref s) => write!(f, "literal: {}", s),
            &Token::Bracket(ref b) => write!(f, "bracket: {:?}", b),
            &Token::Number(ref n) => write!(f, "number: {}", n),
            &Token::Comment(ref s) => write!(f, "comment: {}", s),
            &Token::KeyWord(ref k) => write!(f, "keywords: {:?}", k),
            &Token::Operator(ref o) => write!(f, "operators: {:?}", o),
            &Token::Preprocessor(ref p) => write!(f, "preprocessor: {}", p),
            &Token::Variable(ref v) => write!(f, "variable: {}", v),
        }
    }
}

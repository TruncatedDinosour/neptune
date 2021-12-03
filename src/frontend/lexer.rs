use logos::{Logos, Lexer};

/// Skip a line comment
fn skip_line_comment(lex: &mut Lexer<TokenKind>) -> bool {
    let remainder = lex.remainder().as_bytes();
    let mut cursor: usize = 0;

    loop  {
        if remainder[cursor] == b'\n'.into() {
            cursor += 1;
            break
        }
    }

    lex.bump(cursor);
    true
}

#[derive(Logos, Debug, PartialEq)]
pub enum TokenKind {
    #[token("(")]
    LParen,

    #[token(")")]
    RParen,

    #[token("[")]
    LBrack,

    #[token("]")]
    RBrack,

    #[token("{")]
    LBrace,

    #[token("}")]
    RBrace,

    #[token("+")]
    Plus,

    #[token("-")]
    Minus,

    #[token("*")]
    Star,

    #[token("/")]
    Slash,

    #[token(".")]
    Dot,

    #[token(",")]
    Comma,

    #[token("=")]
    Equal,

    #[token("==")]
    EqualEqual,

    #[token("!=")]
    BangEqual,

    #[token("<")]
    Less,

    #[token(">")]
    Greater,

    #[token("<=")]
    LesserEqual,

    #[token(">=")]
    GreaterEqual,

    #[token(":=")]
    ColonEqual,

    // Type seperator
    #[token("::")]
    ColonColon,

    // Function return type
    #[token("=>")]
    Arrow,

    // Comments
    #[token("#", skip_line_comment)]
    Comment,

    // Identifier
    #[regex(r"[A-Za-z_][A-Za-z0-9_]+")]
    Identifier,

    // Integers
    #[regex(r"[0-9]+")]
    Integer,

    // String and character literals
    #[regex(r"'.'")]
    Char,

    #[regex(r#""((\\"|\\\\)|[^\\"])*""#)]
    String,

    // Binary format integer
    #[regex(r"0(b|B)[0|1]+")]
    BinInteger,

    // Hexadecimal format integer
    #[regex(r"0(x|X)[0-9A-F]+")]
    HexInteger,

    // Keywords (more will be added)
    #[token("if")]
    If,

    #[token("for")]
    For,

    #[token("def")]
    Def,

    #[token("fun")]
    Fun,

    // Type keywords
    // TODO: is there a better way to represent them?
    #[token("int")]
    IntType,

    #[token("str")]
    StrType,

    #[token("bool")]
    BoolType,    

    // Newlines and line continuations
    #[token("\\\n", logos::skip)]
    Continuation,

    #[token("\n")]
    NewLine,

    #[error]
    #[regex(r"[\t\f\s]+", logos::skip)]
    Error
}

#[cfg(test)]
/// Tests for the lexer
mod tests {
    use logos::Logos;
    use crate::frontend::lexer::TokenKind;

    #[test]
    /// Test to make sure we can lex a simple decimal integer
    fn dec_literal() {
        let mut lexer = TokenKind::lexer("123");
        let next = &lexer.next();

        assert_eq!(*next.clone(), Some(TokenKind::Integer));
    }

    #[test]
    /// Test to make sure we can lex a simple hexadecimal integer
    fn hex_literal() {
        let mut lexer = TokenKind::lexer("0x123f");
        let next = &lexer.next();

        assert_eq!(*next.clone(), Some(TokenKind::HexInteger));
    }

    #[test]
    /// Test to make sure we can lex a simple binary integer
    fn bin_literal() {
        let mut lexer = TokenKind::lexer("0b0010");
        let next = &lexer.next();

        assert_eq!(*next.clone(), Some(TokenKind::BinInteger));
    }

    // String and character tests
    #[test]
    fn string_literal() {
        let mut lexer = TokenKind::lexer(r#""hello world""#);
        let next = &lexer.next();

        assert_eq!(*next.clone(), Some(TokenKind::String));
    }

    #[test]
    fn char_literal() {
        let mut lexer = TokenKind::lexer("'A'");
        let next = &lexer.next();

        assert_eq!(*next.clone(), Some(TokenKind::Char));
    }

    /// Test to make sure we can lex an identifier containing a number
    /// e.g foo1bar2
    #[test]
    fn identifier_with_number() {
        let mut lexer = TokenKind::lexer("foo1bar2");
        let next = &lexer.next();

        assert_eq!(*next.clone(), Some(TokenKind::Identifier));
    }

    /// Tets to make sure we can lex an identifier containing underscores
    /// in its name
    #[test]
    fn identifier_with_underscores() {
        let mut lexer = TokenKind::lexer("foo_bar_baz");
        let next = &lexer.next();

        assert_eq!(*next.clone(), Some(TokenKind::Identifier));
    }
}
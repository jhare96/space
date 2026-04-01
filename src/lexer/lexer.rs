use std::sync::OnceLock;

use regex::Regex;

use crate::lexer::keywords::RESERVED_SYMBOLS;
use crate::lexer::token::{Span, Token, TokenKind};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LexError {
    pub message: String,
    pub span: Span,
}

pub fn lex(input: &str) -> Result<Vec<Token>, LexError> {
    let regex = token_regex();
    let mut tokens = Vec::new();
    let mut offset = 0;
    let mut line = 1;
    let mut column = 1;

    while offset < input.len() {
        let captures = regex.captures_at(input, offset).ok_or_else(|| LexError {
            message: "unrecognized token".to_string(),
            span: Span {
                start: offset,
                end: offset,
                line,
                column,
            },
        })?;

        let matched = captures.get(0).ok_or_else(|| LexError {
            message: "regex did not return a full match".to_string(),
            span: Span {
                start: offset,
                end: offset,
                line,
                column,
            },
        })?;

        if matched.start() != offset {
            return Err(LexError {
                message: "unrecognized token".to_string(),
                span: Span {
                    start: offset,
                    end: offset,
                    line,
                    column,
                },
            });
        }

        let lexeme = matched.as_str();
        let span = Span {
            start: matched.start(),
            end: matched.end(),
            line,
            column,
        };

        let kind = if captures.name("whitespace").is_some() {
            advance_position(lexeme, &mut line, &mut column);
            offset = matched.end();
            continue;
        } else if captures.name("comment").is_some() {
            TokenKind::Comment
        } else if captures.name("string").is_some() {
            TokenKind::String
        } else if captures.name("number").is_some() {
            TokenKind::Number
        } else if captures.name("identifier").is_some() {
            TokenKind::from_keyword(lexeme).unwrap_or(TokenKind::Identifier)
        } else if captures.name("symbol").is_some() {
            TokenKind::from_symbol(lexeme).ok_or_else(|| LexError {
                message: format!("unknown symbol: {lexeme}"),
                span,
            })?
        } else {
            return Err(LexError {
                message: "regex matched an unsupported token class".to_string(),
                span,
            });
        };

        tokens.push(Token {
            kind,
            lexeme: lexeme.to_string(),
            span,
        });

        advance_position(lexeme, &mut line, &mut column);
        offset = matched.end();
    }

    Ok(tokens)
}

fn token_regex() -> &'static Regex {
    static TOKEN_REGEX: OnceLock<Regex> = OnceLock::new();

    TOKEN_REGEX.get_or_init(|| {
        let mut symbols: Vec<_> = RESERVED_SYMBOLS.iter().copied().collect();
        symbols.sort_by_key(|symbol| std::cmp::Reverse(symbol.len()));
        let symbol_pattern = symbols
            .into_iter()
            .map(regex::escape)
            .collect::<Vec<_>>()
            .join("|");

        let pattern = format!(
            r#"(?xms)
            (?P<whitespace>\s+)
            |(?P<comment>//[^\r\n]*|//\*.*?\*/|/\*.*?\*/)
            |(?P<string>"(?:\\.|[^"\\])*")
            |(?P<number>\d+(?:\.\d+)?(?:[eE][+-]?\d+)?)
            |(?P<identifier>[A-Za-z_][A-Za-z0-9_]*)
            |(?P<symbol>{symbol_pattern})
            "#
        );

        Regex::new(&pattern).expect("token regex must compile")
    })
}

fn advance_position(lexeme: &str, line: &mut usize, column: &mut usize) {
    for ch in lexeme.chars() {
        if ch == '\n' {
            *line += 1;
            *column = 1;
        } else {
            *column += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::token::TokenKind;

    #[test]
    fn lexes_keywords_symbols_and_identifiers() {
        let tokens = lex("package Demo { part x; }").unwrap();
        let kinds: Vec<_> = tokens.iter().map(|token| token.kind).collect();

        assert_eq!(
            kinds,
            vec![
                TokenKind::Package,
                TokenKind::Identifier,
                TokenKind::LBrace,
                TokenKind::Part,
                TokenKind::Identifier,
                TokenKind::SemiColon,
                TokenKind::RBrace,
            ]
        );
    }

    #[test]
    fn uses_span_positions() {
        let tokens = lex("part x\nstate y").unwrap();

        assert_eq!(tokens[0].span.line, 1);
        assert_eq!(tokens[0].span.column, 1);
        assert_eq!(tokens[2].span.line, 2);
        assert_eq!(tokens[2].span.column, 1);
    }
}
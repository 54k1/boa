//! Lexical declaration parsing.
//!
//! This parses `let` and `const` declarations.
//!
//! More information:
//!  - [ECMAScript specification][spec]
//!
//! [spec]: https://tc39.es/ecma262/#sec-let-and-const-declarations

use crate::{
    syntax::{
        ast::{keyword::Keyword, node::Node, punc::Punctuator, token::TokenKind},
        parser::{
            expression::Initializer, AllowAwait, AllowIn, AllowYield, Cursor, ParseError,
            ParseResult, TokenParser,
        },
    },
    Interner,
};

/// Parses a lexical declaration.
///
/// More information:
///  - [ECMAScript specification][spec]
///
/// [spec]: https://tc39.es/ecma262/#prod-LexicalDeclaration
#[derive(Debug, Clone, Copy)]
pub(super) struct LexicalDeclaration {
    allow_in: AllowIn,
    allow_yield: AllowYield,
    allow_await: AllowAwait,
}

impl LexicalDeclaration {
    /// Creates a new `LexicalDeclaration` parser.
    pub(super) fn new<I, Y, A>(allow_in: I, allow_yield: Y, allow_await: A) -> Self
    where
        I: Into<AllowIn>,
        Y: Into<AllowYield>,
        A: Into<AllowAwait>,
    {
        Self {
            allow_in: allow_in.into(),
            allow_yield: allow_yield.into(),
            allow_await: allow_await.into(),
        }
    }
}

impl TokenParser for LexicalDeclaration {
    type Output = Node;

    fn parse(self, cursor: &mut Cursor<'_>, interner: &mut Interner) -> ParseResult {
        let tok = cursor.next().ok_or(ParseError::AbruptEnd)?;

        match tok.kind {
            TokenKind::Keyword(Keyword::Const) => {
                BindingList::new(self.allow_in, self.allow_yield, self.allow_await, true)
                    .parse(cursor, interner)
            }
            TokenKind::Keyword(Keyword::Let) => {
                BindingList::new(self.allow_in, self.allow_yield, self.allow_await, false)
                    .parse(cursor, interner)
            }
            _ => unreachable!("unknown token found"),
        }
    }
}

/// Parses a binding list.
///
/// It will return an error if a `const` declaration is being parsed and there is no
/// initializer.
///
/// More information: <https://tc39.es/ecma262/#prod-BindingList>.
#[derive(Debug, Clone, Copy)]
struct BindingList {
    allow_in: AllowIn,
    allow_yield: AllowYield,
    allow_await: AllowAwait,
    is_const: bool,
}

impl BindingList {
    /// Creates a new `BindingList` parser.
    fn new<I, Y, A>(allow_in: I, allow_yield: Y, allow_await: A, is_const: bool) -> Self
    where
        I: Into<AllowIn>,
        Y: Into<AllowYield>,
        A: Into<AllowAwait>,
    {
        Self {
            allow_in: allow_in.into(),
            allow_yield: allow_yield.into(),
            allow_await: allow_await.into(),
            is_const,
        }
    }
}

impl TokenParser for BindingList {
    type Output = Node;

    fn parse(self, cursor: &mut Cursor<'_>, interner: &mut Interner) -> ParseResult {
        // Create vectors to store the variable declarations
        // Const and Let signatures are slightly different, Const needs definitions, Lets don't
        let mut let_decls = Vec::new();
        let mut const_decls = Vec::new();

        loop {
            let token = cursor.next().ok_or(ParseError::AbruptEnd)?;
            let name = if let TokenKind::Identifier(name) = token.kind {
                name
            } else {
                return Err(ParseError::expected(
                    vec![String::from("identifier")],
                    token.display(interner).to_string(),
                    token.pos,
                    if self.is_const {
                        "const declaration"
                    } else {
                        "let declaration"
                    },
                ));
            };

            match cursor.peek(0) {
                Some(token) if token.kind == TokenKind::Punctuator(Punctuator::Assign) => {
                    let init = Some(
                        Initializer::new(self.allow_in, self.allow_yield, self.allow_await)
                            .parse(cursor, interner)?,
                    );
                    if self.is_const {
                        const_decls.push((name, init.unwrap()));
                    } else {
                        let_decls.push((name, init));
                    };
                }
                _ => {
                    if self.is_const {
                        let next_tok = cursor.next().ok_or(ParseError::AbruptEnd)?;
                        return Err(ParseError::expected(
                            vec![Punctuator::Assign.to_string()],
                            next_tok.display(interner).to_string(),
                            next_tok.pos,
                            "const declaration",
                        ));
                    } else {
                        let_decls.push((name, None));
                    }
                }
            }

            match cursor.peek_semicolon(false) {
                (true, _) => break,
                (false, Some(tk)) if tk.kind == TokenKind::Punctuator(Punctuator::Comma) => {
                    let _ = cursor.next();
                }
                _ => {
                    let next_tok = cursor.next().ok_or(ParseError::AbruptEnd)?;
                    return Err(ParseError::expected(
                        vec![
                            Punctuator::Semicolon.to_string(),
                            TokenKind::LineTerminator.display(interner).to_string(),
                        ],
                        next_tok.display(interner).to_string(),
                        next_tok.pos,
                        "lexical declaration",
                    ));
                }
            }
        }

        if self.is_const {
            Ok(Node::ConstDecl(const_decls))
        } else {
            Ok(Node::LetDecl(let_decls))
        }
    }
}

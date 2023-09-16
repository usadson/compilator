// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

mod keyword;
mod preprocessor;
mod punctuator;

pub use self::{
    keyword::{
        IdentifierIsNotAKeyword,
        Keyword,
    },
    preprocessor::{
        PreprocessorToken,
        PreprocessorTokenKind,
        PreprocessorTokenNotMappedToTokenError,
    },
    punctuator::Punctuator,
};

#[derive(Debug, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub start: usize,
    pub end: usize,
}

#[derive(Debug, PartialEq)]
pub enum TokenKind {
    Keyword(Keyword),
    Identifier(String),
    Constant,
    StringLiteral(String),
    Punctuator(Punctuator),
}

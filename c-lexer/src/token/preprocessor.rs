// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use std::{
    error::Error,
    fmt::Display,
};

use crate::{
    Punctuator,
    Token,
    TokenKind,
};

#[derive(Debug, PartialEq)]
pub struct PreprocessorToken {
    pub kind: PreprocessorTokenKind,
    pub start: usize,
    pub end: usize,
}

impl TryFrom<PreprocessorToken> for Token {
    type Error = PreprocessorTokenNotMappedToTokenError;

    fn try_from(value: PreprocessorToken) -> Result<Self, Self::Error> {
        Ok(Self {
            kind: value.kind.try_into()?,
            start: value.start,
            end: value.end,
        })
    }
}

#[derive(Debug, PartialEq)]
pub enum PreprocessorTokenKind {
    Whitespace(char),
    HeaderName,
    Identifier(String),
    PpNumber,
    CharacterConstant,
    StringLiteral(String),
    Punctuator(Punctuator),

    /// ?
    OtherUniversalCharacterName,

    /// >
    NonWhiteSpaceCharacter(char),
}

impl TryFrom<PreprocessorTokenKind> for TokenKind {
    type Error = PreprocessorTokenNotMappedToTokenError;

    fn try_from(value: PreprocessorTokenKind) -> Result<TokenKind, Self::Error> {
        match value {
            PreprocessorTokenKind::Whitespace(..) => Err(PreprocessorTokenNotMappedToTokenError),
            PreprocessorTokenKind::HeaderName => Err(PreprocessorTokenNotMappedToTokenError),
            PreprocessorTokenKind::Identifier(ident) => Ok(TokenKind::Identifier(ident)),
            PreprocessorTokenKind::PpNumber => todo!(),
            PreprocessorTokenKind::CharacterConstant => todo!(),
            PreprocessorTokenKind::StringLiteral(string_literal) => Ok(TokenKind::StringLiteral(string_literal)),
            PreprocessorTokenKind::Punctuator(punctuator) => Ok(TokenKind::Punctuator(punctuator)),

            PreprocessorTokenKind::OtherUniversalCharacterName => todo!(),
            PreprocessorTokenKind::NonWhiteSpaceCharacter(c) => todo!("unsupported char: {c} U+{:X}", c as u32),
        }
    }
}

/// Not every [`PreprocessorTokenKind`] can be converted to a [`TokenKind`], as
/// specified in ISO/IEC 9899 6.4.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct PreprocessorTokenNotMappedToTokenError;

impl Display for PreprocessorTokenNotMappedToTokenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("PreprocessorTokenNotMappedToTokenError")
    }
}

impl Error for PreprocessorTokenNotMappedToTokenError {
}

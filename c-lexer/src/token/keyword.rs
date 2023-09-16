// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use std::{
    error::Error,
    fmt::Display, str::FromStr,
};

/// These can be found in appendix A.1.2
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[derive(strum::AsRefStr, strum::EnumIter)]
#[strum(serialize_all = "snake_case")]
pub enum Keyword {
    Bool,
    Char,
    Int,
    Void,
}

impl FromStr for Keyword {
    type Err = IdentifierIsNotAKeyword;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use strum::IntoEnumIterator;

        Self::iter()
            .find(|x| x.as_ref() == s)
            .ok_or(IdentifierIsNotAKeyword)
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct IdentifierIsNotAKeyword;

impl Display for IdentifierIsNotAKeyword {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("IdentifierIsNotAKeyword")
    }
}

impl Error for IdentifierIsNotAKeyword {
}

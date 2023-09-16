// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use std::str::CharIndices;

/// Helps orchestrate the intake of characters by the [`Lexer`][crate::Lexer],
/// whilst providing a peekable interface.
#[derive(Clone, Debug)]
pub(super) struct SourceCode<'source_code> {
    str: &'source_code str,
    data: CharIndices<'source_code>,
    previous: Option<(usize, char)>,
    current: Option<(usize, char)>,
    peeked_char: Option<(usize, char)>,
}

impl<'source_code> SourceCode<'source_code> {
    pub const fn index(&self) -> usize {
        if let Some((index, _)) = self.peeked_char {
            return index;
        }

        if let Some((index, char)) = self.current {
            return index + char.len_utf8();
        }

        let Some((previous_index, previous_char)) = self.previous else {
            debug_assert!(self.str.is_empty());
            return 0;
        };

        previous_index + previous_char.len_utf8()
    }

    pub const fn as_str(&self) -> &'source_code str {
        self.str
    }

    pub fn peek(&self) -> Option<char> {
        self.peeked_char.map(|tuple| tuple.1)
    }
}

impl<'source_code> From<&'source_code str> for SourceCode<'source_code> {
    fn from(value: &'source_code str) -> Self {
        let mut iter = value.char_indices();
        let peeked_char = iter.next();
        Self {
            str: value,
            data: iter,
            current: None,
            peeked_char,
            previous: None,
        }
    }
}

impl<'source_code> Iterator for SourceCode<'source_code> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        let peeked = self.peeked_char
            .or_else(|| self.data.next());

        self.previous = self.current;
        self.current = peeked;
        self.peeked_char = self.data.next();

        let (_, current) = self.current?;
        Some(current)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_file() {
        let mut instance = SourceCode::from("");
        assert_eq!(instance.index(), 0);
        assert_eq!(instance.peek(), None);
        assert_eq!(instance.next(), None);
        assert_eq!(instance.peek(), None);
        assert_eq!(instance.index(), 0);
    }

    #[test]
    fn space() {
        let mut instance = SourceCode::from(" ");
        assert_eq!(instance.index(), 0);
        assert_eq!(instance.peek(), Some(' '));
        assert_eq!(instance.next(), Some(' '));
        assert_eq!(instance.peek(), None);
        assert_eq!(instance.index(), 1, "Invalid: {instance:#?}");
    }

    #[test]
    fn ident_main() {
        let mut instance = SourceCode::from("main");
        assert_eq!(instance.index(), 0);
        assert_eq!(instance.peek(), Some('m'));

        assert_eq!(instance.next(), Some('m'));
        assert_eq!(instance.index(), 1);
        assert_eq!(instance.peek(), Some('a'));

        assert_eq!(instance.next(), Some('a'));
        assert_eq!(instance.index(), 2);
        assert_eq!(instance.peek(), Some('i'));

        assert_eq!(instance.next(), Some('i'));
        assert_eq!(instance.index(), 3);
        assert_eq!(instance.peek(), Some('n'));

        assert_eq!(instance.next(), Some('n'));
        assert_eq!(instance.index(), 4);
        assert_eq!(instance.peek(), None);

        assert_eq!(instance.next(), None);
    }
}

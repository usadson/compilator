// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

mod source_code;

use crate::{
    PreprocessorToken,
    PreprocessorTokenKind, Punctuator,
};

use self::source_code::SourceCode;

pub struct Lexer<'source_code> {
    source_code: SourceCode<'source_code>,
}

impl<'source_code> Lexer<'source_code> {
    pub fn new(source_code: &'source_code str) -> Self {
        Self {
            source_code: source_code.into()
        }
    }

    fn consume_single_char_token(
        &mut self,
        kind: PreprocessorTokenKind,
    ) -> Option<PreprocessorToken> {
        let start = self.source_code.index();
        _ = self.source_code.next()?;
        let end = self.source_code.index();

        Some(PreprocessorToken {
            kind,
            start,
            end,
        })
    }

    fn consume_single_char_punctuator(
        &mut self,
        punctuator: Punctuator,
    ) -> Option<PreprocessorToken> {
        let kind = PreprocessorTokenKind::Punctuator(punctuator);
        self.consume_single_char_token(kind)
    }

    fn consume_two_possible_punctuators(
        &mut self,
        char: char,
        single: Punctuator,
        double: Punctuator
    ) -> PreprocessorToken {
        let start = self.source_code.index();
        _ = self.source_code.next();

        if self.source_code.peek() == Some(char) {
            _ = self.source_code.next();

            return PreprocessorToken {
                kind: PreprocessorTokenKind::Punctuator(double),
                start,
                end: self.source_code.index(),
            };
        }

        PreprocessorToken {
            kind: PreprocessorTokenKind::Punctuator(single),
            start,
            end: self.source_code.index(),
        }
    }

    fn consume_three_possible_punctuators(
        &mut self,
        default: Punctuator,
        second_char: char,
        second_kind: Option<Punctuator>,
        third_char: char,
        third_kind: Punctuator,
    ) -> PreprocessorToken {
        let start = self.source_code.index();

        _ = self.source_code.next();
        let source_code_reset = self.source_code.clone();

        if self.source_code.peek() == Some(second_char) {
            _ = self.source_code.next();

            if self.source_code.peek() == Some(third_char) {
                _ = self.source_code.next();
                let kind = PreprocessorTokenKind::Punctuator(third_kind);
                let end = self.source_code.index();

                return PreprocessorToken {
                    kind,
                    start,
                    end,
                };
            }

            if let Some(second_kind) = second_kind {
                let kind = PreprocessorTokenKind::Punctuator(second_kind);
                let end = self.source_code.index();

                return PreprocessorToken {
                    kind,
                    start,
                    end,
                };
            }
        }

        self.source_code = source_code_reset;

        let kind = PreprocessorTokenKind::Punctuator(default);
        let end = self.source_code.index();
        PreprocessorToken { kind, start, end }
    }

    fn consume_math_or_affix_or_assignment(
        &mut self,
        char: char,
        math: Punctuator,
        affix: Punctuator,
        assignment: Punctuator
    ) -> PreprocessorToken {
        let start = self.source_code.index();
        _ = self.source_code.next();

        if self.source_code.peek() == Some(char) {
            _ = self.source_code.next();
            let end = self.source_code.index();
            let kind = PreprocessorTokenKind::Punctuator(affix);
            return PreprocessorToken { kind, start, end };
        }

        if self.source_code.peek() == Some('=') {
            _ = self.source_code.next();
            let end = self.source_code.index();
            let kind = PreprocessorTokenKind::Punctuator(assignment);
            return PreprocessorToken { kind, start, end };
        }

        let end = self.source_code.index();
        let kind = PreprocessorTokenKind::Punctuator(math);
        PreprocessorToken { kind, start, end }
    }

    fn consume_identifier_or_keyword(&mut self) -> PreprocessorToken {
        let start = self.source_code.index();

        while let Some(token) = self.source_code.peek() {
            if !matches!(token, '_' | 'a'..='z' | 'A'..='Z' | '0'..='9') {
                break;
            }

            _ = self.source_code.next();
        }

        let end = self.source_code.index();
        debug_assert_ne!(start, end);

        let str = &self.source_code.as_str()[start..end];

        PreprocessorToken {
            kind: PreprocessorTokenKind::Identifier(str.into()),
            start,
            end,
        }
    }
}

impl<'source_code> Iterator for Lexer<'source_code> {
    type Item = PreprocessorToken;

    fn next(&mut self) -> Option<Self::Item> {
        let start = self.source_code.index();
        let first_char = self.source_code.peek()?;

        match first_char {
            ' ' | '\n' | '\t' | '\r' => self.consume_single_char_token(PreprocessorTokenKind::Whitespace(first_char)),

            '[' => self.consume_single_char_punctuator(Punctuator::LeftSquareBracket),
            ']' => self.consume_single_char_punctuator(Punctuator::LeftSquareBracket),

            '(' => self.consume_single_char_punctuator(Punctuator::LeftParenthesis),
            ')' => self.consume_single_char_punctuator(Punctuator::RightParenthesis),

            '{' => self.consume_single_char_punctuator(Punctuator::LeftCurlyBracket),
            '}' => self.consume_single_char_punctuator(Punctuator::RightCurlyBracket),

            '.' => Some(self.consume_three_possible_punctuators(
                    Punctuator::FullStop,
                    '.',
                    None,
                    '.',
                    Punctuator::Ellipsis,
            )),

            '-' => {
                let reset_source_code = self.source_code.clone();
                _ = self.source_code.next();

                if self.source_code.peek() == Some('>') {
                    _ = self.source_code.next();
                    let kind = PreprocessorTokenKind::Punctuator(Punctuator::PointerMemberAccessOperator);
                    let end = self.source_code.index();
                    return Some(PreprocessorToken { kind, start, end });
                }

                self.source_code = reset_source_code;

                Some(self.consume_math_or_affix_or_assignment(
                    '-',
                    Punctuator::Minus,
                    Punctuator::DecrementOperator,
                    Punctuator::SubtractAssign,
                ))
            }

            '+' => Some(self.consume_math_or_affix_or_assignment(
                '+',
                Punctuator::PlusSign,
                Punctuator::IncrementOperator,
                Punctuator::AddAssign,
            )),

            '~' => self.consume_single_char_punctuator(Punctuator::Tilde),

            '!' => Some(self.consume_two_possible_punctuators(
                '=',
                Punctuator::ExclamationMark,
                Punctuator::NotEqualTo,
            )),

            '/' => Some(self.consume_two_possible_punctuators(
                '=',
                Punctuator::Solidus,
                Punctuator::DivideAssign,
            )),

            '*' => Some(self.consume_two_possible_punctuators(
                '=',
                Punctuator::Asterisk,
                Punctuator::MultiplyAssign,
            )),

            '%' => Some(self.consume_two_possible_punctuators(
                '=',
                Punctuator::Percentage,
                Punctuator::ModuloAssign,
            )),

            '|' => Some(self.consume_math_or_affix_or_assignment(
                '|',
                Punctuator::BitwiseOr,
                Punctuator::LogicalOr,
                Punctuator::BitwiseOrAssign,
            )),

            '&' => Some(self.consume_math_or_affix_or_assignment(
                '&',
                Punctuator::ReferenceOperatorOrBitwiseAnd,
                Punctuator::LogicalAnd,
                Punctuator::BitwiseAndAssign,
            )),

            '?' => self.consume_single_char_punctuator(Punctuator::QuestionMark),

            ':' => Some(self.consume_two_possible_punctuators(
                ':',
                Punctuator::Colon,
                Punctuator::DoubleColon,
            )),

            ';' => self.consume_single_char_punctuator(Punctuator::Semicolon),

            '=' => Some(self.consume_two_possible_punctuators(
                '=',
                Punctuator::EqualsSign,
                Punctuator::EqualTo,
            )),

            '<' => {
                let start = self.source_code.index();
                let reset_source_code = self.source_code.clone();
                {
                    _ = self.source_code.next();

                    if self.source_code.peek() == Some('=') {
                        _ = self.source_code.next();
                        return Some(PreprocessorToken {
                            kind: PreprocessorTokenKind::Punctuator(Punctuator::LessThanOrEqualTo),
                            start,
                            end: self.source_code.index(),
                        })
                    }
                }

                self.source_code = reset_source_code;

                Some(self.consume_three_possible_punctuators(
                    Punctuator::LessThan,
                    '<',
                    Some(Punctuator::LeftBitShift),
                    '=',
                    Punctuator::LeftBitShiftAssign,
                ))
            }

            '>' => {
                let start = self.source_code.index();
                let reset_source_code = self.source_code.clone();
                {
                    _ = self.source_code.next();

                    if self.source_code.peek() == Some('=') {
                        _ = self.source_code.next();
                        return Some(PreprocessorToken {
                            kind: PreprocessorTokenKind::Punctuator(Punctuator::GreaterThanOrEqualTo),
                            start,
                            end: self.source_code.index(),
                        })
                    }
                }

                self.source_code = reset_source_code;

                Some(self.consume_three_possible_punctuators(
                    Punctuator::GreaterThan,
                    '>',
                    Some(Punctuator::RightBitShift),
                    '=',
                    Punctuator::RightBitShiftAssign,
                ))
            }

            '^' => Some(self.consume_two_possible_punctuators(
                '=',
                Punctuator::BitwiseXor,
                Punctuator::BitwiseXorAssign,
            )),

            '#' => Some(self.consume_two_possible_punctuators(
                '#',
                Punctuator::Pound,
                Punctuator::DoublePound,
            )),

            '_' | 'a'..='z' | 'A'..='Z' => Some(self.consume_identifier_or_keyword()),

            _ => {
                eprintln!("[lexer] Unknown token: {first_char} U+{:X}", first_char as u32);

                self.consume_single_char_token(
                    PreprocessorTokenKind::NonWhiteSpaceCharacter(first_char)
                )
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;
    use super::*;
    use pretty_assertions::assert_eq;

    #[rstest]
    #[case(" ", PreprocessorTokenKind::Whitespace(' '))]
    #[case("\r", PreprocessorTokenKind::Whitespace('\r'))]
    #[case("\n", PreprocessorTokenKind::Whitespace('\n'))]
    #[case("\t", PreprocessorTokenKind::Whitespace('\t'))]
    #[case("main", PreprocessorTokenKind::Identifier("main".into()))]
    #[case("&&", PreprocessorTokenKind::Punctuator(Punctuator::LogicalAnd))]
    #[case("<<=", PreprocessorTokenKind::Punctuator(Punctuator::LeftBitShiftAssign))]
    fn single(#[case] input: &str, #[case] expected_kind: PreprocessorTokenKind) {
        let mut lexer = Lexer::new(input);
        let token = lexer.next();
        let token_kind = token.as_ref().map(|token| &token.kind);
        assert_eq!(token_kind, Some(&expected_kind));
        assert_eq!(lexer.next(), None);
    }

    #[rstest]
    #[case(
        "    ",
        &[
            PreprocessorTokenKind::Whitespace(' '),
            PreprocessorTokenKind::Whitespace(' '),
            PreprocessorTokenKind::Whitespace(' '),
            PreprocessorTokenKind::Whitespace(' '),
        ]
    )]
    #[case(
        "int main",
        &[

            PreprocessorTokenKind::Identifier("int".into()),
            PreprocessorTokenKind::Whitespace(' '),
            PreprocessorTokenKind::Identifier("main".into()),
        ]
    )]
    #[case(
        "int main()",
        &[

            PreprocessorTokenKind::Identifier("int".into()),
            PreprocessorTokenKind::Whitespace(' '),
            PreprocessorTokenKind::Identifier("main".into()),
            PreprocessorTokenKind::Punctuator(Punctuator::LeftParenthesis),
            PreprocessorTokenKind::Punctuator(Punctuator::RightParenthesis),
        ]
    )]
    #[case(
"int main() {

}",
        &[
            PreprocessorTokenKind::Identifier("int".into()),
            PreprocessorTokenKind::Whitespace(' '),
            PreprocessorTokenKind::Identifier("main".into()),
            PreprocessorTokenKind::Punctuator(Punctuator::LeftParenthesis),
            PreprocessorTokenKind::Punctuator(Punctuator::RightParenthesis),
            PreprocessorTokenKind::Whitespace(' '),
            PreprocessorTokenKind::Punctuator(Punctuator::LeftCurlyBracket),
            PreprocessorTokenKind::Whitespace('\n'),
            PreprocessorTokenKind::Whitespace('\n'),
            PreprocessorTokenKind::Punctuator(Punctuator::RightCurlyBracket),
        ]
    )]
    fn excerpt(#[case] input: &str, #[case] expected: &[PreprocessorTokenKind]) {
        let actual: Vec<_> = Lexer::new(input)
            .map(|token| token.kind)
            .collect();

        assert_eq!(expected, actual);
    }
}

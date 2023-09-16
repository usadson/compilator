// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Punctuator {
    LeftSquareBracket,
    RightSquareBracket,

    LeftParenthesis,
    RightParenthesis,

    LeftCurlyBracket,
    RightCurlyBracket,

    FullStop,
    PointerMemberAccessOperator,

    IncrementOperator,
    DecrementOperator,

    ReferenceOperatorOrBitwiseAnd,

    Asterisk,
    PlusSign,
    Minus,
    Tilde,
    ExclamationMark,
    Solidus,
    Percentage,

    LeftBitShift,
    RightBitShift,

    LessThan,
    GreaterThan,
    LessThanOrEqualTo,
    GreaterThanOrEqualTo,
    EqualTo,
    NotEqualTo,

    BitwiseXor,
    BitwiseOr,

    LogicalAnd,
    LogicalOr,

    QuestionMark,
    Colon,
    DoubleColon,
    Semicolon,
    Ellipsis,

    EqualsSign,

    MultiplyAssign,
    DivideAssign,
    ModuloAssign,
    AddAssign,
    SubtractAssign,
    LeftBitShiftAssign,
    RightBitShiftAssign,
    BitwiseAndAssign,
    BitwiseXorAssign,
    BitwiseOrAssign,

    Comma,
    Pound,
    DoublePound,
}

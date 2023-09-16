// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use c_lexer::{
    Lexer,
    Token,
};

fn main() {
    let data = std::fs::read_to_string("test.c").unwrap();

    let tokens: Vec<Token> = Lexer::new(&data)
        .filter_map(|x| x.try_into().ok())
        .collect();
    println!("Tokens: {tokens:#?}");
}


use luthor::{Tokenizer, StateFunction};
use luthor::token::{Token, Category};

fn initial_state(lexer: &mut Tokenizer) -> Option<StateFunction> {
    if lexer.has_prefix("::") {
        lexer.tokenize(Category::Text);
        lexer.tokenize_next(2, Category::Text);
    }

    match lexer.current_char() {
        Some(c) => {
            match c {
                ' ' | '\n' | '\t' => {
                    lexer.tokenize(Category::Text);
                    lexer.advance();
                    return Some(StateFunction(whitespace));
                }
                '`' |
                '=' |
                '_' |
                '-' |
                '.' |
                '(' |
                ')' |
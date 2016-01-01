mod dawg;

use niji::context::Context;

use std::option::Option;

//extern crate core;
//use core::marker::PhantomFn;

pub struct Token<'a> {
    pub data: &'a str
}

pub struct ParserToken<'a, 'b> {
    token: &'a Token<'a>,
    context: &'b Context,
    url: Option<String>
}

pub enum ParseTokenResult<'a, 'b> {
    TokenList(Vec<ParserToken<'a, 'b>>),
    Token(ParserToken<'a, 'b>),
    None
}

pub trait Parser<'a, 'b> /* : core::marker::PhantomFn<Self> */ {

    fn parse_token(self: &mut Self, token: &'a Token, context: &'b Context, url: Option<String>) -> ParseTokenResult<'a, 'b> {
        ParseTokenResult::Token( ParserToken { token: token, context: context, url: url } )
    }

}

use niji::parser::Token;

mod plain;
pub use self::plain::PlaintextRenderer;
//mod tex;
//pub use self::tex::TeXRenderer;
mod html;
pub use self::html::HTMLRenderer;
//mod xml;
//pub use self::xml::XMLRenderer;

use std::string::String;

pub trait Renderer {

    fn get_header(&self) -> String {
        "".to_string()
    }

    fn get_footer(&self) -> String {
        "".to_string()
    }

    fn parse_token(&self, token: Token) -> String {
        token.data.to_string()
    }

}

pub mod context;
pub mod parser;
pub mod renderer;
pub mod theme;

// Re-export sub-module
use self::context::*;
use self::parser::*;
use self::theme::*;

pub struct Niji<'a, 'b> {
    lang: &'a context::Language,
    renderer: &'b renderer::Renderer
}

impl<'a, 'b> Niji<'a, 'b> {

    pub fn new (lang: &'a mut context::Language, renderer: &'b mut renderer::Renderer) -> Niji<'a, 'b> {
        Niji { lang: lang, renderer: renderer }
    }

    pub fn format(&self, s: &str) -> String {
        "I can haz formatted?".to_string()
    }

}

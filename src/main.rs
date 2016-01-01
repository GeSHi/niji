#![feature(str_char, core)]

pub mod niji;

//extern crate unicode_normalization;

use std::io::prelude::*;
use std::fs::File;
use std::str::CharRange;

use niji::Niji;
use niji::context::Language;
use niji::renderer::HTMLRenderer;
use niji::theme::Theme;

fn main() {
    fn foo() -> std::io::Result<()> {
        let mut f = try!(File::open("input.txt"));
        let mut s = String::new();
        try!(f.read_to_string(&mut s));

        let mut niji_language = niji::context::Language::new();
        niji_language.loadFromSpec("text");

        let mut niji_theme = niji::theme::Theme::new();
        niji_theme.loadFromSpec("default");

        let mut niji_format_raw = niji::renderer::HTMLRenderer::new();
        let mut niji_format : &mut niji::renderer::Renderer = &mut niji_format_raw;

        let mut niji = Niji::new(&mut niji_language, niji_format);
        let formatted = niji.format(&s);

        println!("{}\n", formatted);

        let mut i = 0;
        while i < formatted.len() {
            let CharRange {ch, next} = formatted.char_range_at(i);
            println!("{}: {}", i, ch);
            i = next;
        }

        Ok(())
    }

    let r = foo();
    match r {
        Ok(_) => println!("OK!"),
        _ => println!("Done!")
    }
}

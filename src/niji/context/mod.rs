use niji::theme::Styler;

use std::ptr;
use std::cell::RefCell;
use std::rc::Rc;
use std::rc::Weak;
use std::vec;

type size = u64;

pub struct Language<'a> {
    name: &'a str,
    root: Rc<Box<Context>>
}

pub struct StartData<'a> {
    start: size,
    length: size,
    key: u64,
    delim: &'a str,
}

pub struct EndData<'a> {
    start: size,
    length: size,
    delim: &'a str,
}

pub trait Context {

    fn get_name(&self) -> String;

    fn get_language(&self) -> Rc<Language>;

    fn is_alias(&self) -> bool;

    fn can_start(&self, code: &str) -> bool {
        match self.ctx_start_data(&code) {
            Some(_) => true,
            None => false
        }
    }

    fn ctx_start_data(&self, code: &str) -> Option<StartData>;
    fn ctx_end_data(&self, start: StartData) -> Option<EndData>;

}

impl<'a> Language<'a> {

    pub fn new() -> Language<'a> {
        Language::newByName("undefined")
    }

    pub fn newByName(langname: &'a str) -> Language<'a> {
        let mut rc = RootContext::new();
        let mut lang = Language {
            name: langname,
            root: Rc::new( Box::new( rc ) )
        };
        rc.language = Some( Rc::downgrade( & Rc::new( lang ) ) );
        lang
    }

    pub fn loadFromSpec(&self, spec: &str) -> bool {
        true
    }

}

pub struct RootContext<'a, 'b> {
    language: Option<Weak<Language<'a>>>,

    subcontexts: RefCell<Vec<&'b Context>>
}

impl<'a, 'b> RootContext<'a, 'b> {

    pub fn new() -> RootContext<'a, 'b> {
        RootContext {
            language: None,
            subcontexts: RefCell::new( Vec::new() )
        }
    }

}

impl<'a, 'b> Context for RootContext<'a, 'b> {

    fn get_name(&self) -> String {
        ":".to_string() + self.get_language().name + "/"
    }

    fn get_language(&self) -> Rc<Language> {
        let langref = match self.language {
            Some(lang) => lang.upgrade(),
            None => None
        };
        match langref {
            Some(rl) => rl,
            None => Rc::new( Language::new() )
        }
    }

    fn is_alias(&self) -> bool {
        false
    }

    fn ctx_start_data(&self, code: &str) -> Option<StartData> {
        Some( StartData {
            start: 0,
            length: 0,
            key: 0,
            delim: ""
        } )
    }

    fn ctx_end_data(&self, start: StartData) -> Option<EndData> {
        Some( EndData {
            start: start.start + start.length,
            length: 0,
            delim: ""
        } )
    }

}

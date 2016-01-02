use niji::theme::Styler;

use std::ptr;
use std::cell::RefCell;
use std::rc::Rc;
use std::rc::Weak;
use std::vec;

type size = u64;

pub struct Language(Rc<LanguageInner>);

struct LanguageInner {
    name: String,
    root: Rc<RootContext>
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

    /// Returns the fully-qualified name of this context
    fn get_name(&self) -> &str;

    /// Returns a handle to the language associated with this context's
    /// root node
    fn get_language(&self) -> Language;

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

impl Language {

    pub fn new() -> Language {
        Language::newByName("undefined")
    }

    pub fn newByName(langname: &str) -> Language {
        unsafe {
            use std::{mem, ptr};

            let mut lang_rc = Rc::new(LanguageInner {
                name: langname.to_owned(),
                root: Rc::new(mem::uninitialized())
            });

            let mut unsafe_root_mut_ref = {
                let lang_mut = Rc::get_mut(&mut lang_rc).unwrap();
                let root_mut = Rc::get_mut(&mut lang_mut.root).unwrap();
                // What this cast does is give us a mutable pointer
                // to the contents of lang_rc.root, *without* borrowing
                // lang_rc mutably. We need to do this because we're
                // going to borrow lang_rc immutably to get a Weak ref
                // from it. This is safe (i.e. does not violate the
                // aliasing rules) because the *contents* of a Rc are not
                // part of the same object as the Rc itself; they're part
                // of the RcInner which is separately allocated. So while
                // logically it appears that we have have an immutable
                // reference of lang_rc at the same time as a mutable reference
                // inside of it (*** which would be undefined behaviour ***),
                // actually we do not.
                &mut *(root_mut as *mut RootContext)
            };

            ptr::write(unsafe_root_mut_ref, RootContext {
                language: Rc::downgrade(&lang_rc),
                subcontexts: RefCell::new(vec![]),

                full_name: format!(":{}/", langname)
            });

            Language(lang_rc)
        }
    }

    pub fn loadFromSpec(&self, spec: &str) -> bool {
        true
    }

}

pub struct RootContext {
    language: Weak<LanguageInner>,
    subcontexts: RefCell<Vec<Box<Context>>>,

    full_name: String
}

impl Context for RootContext {
    fn get_name(&self) -> &str {
        &self.full_name
    }

    fn get_language(&self) -> Language {
        // This unwrap is OK since the referenced language owns us;
        // if it's gone that we've died and there's no way we're in
        // this function
        Language(self.language.upgrade().unwrap())
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

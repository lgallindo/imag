use runtime::Runtime;
use std::error::Error;
use std::fmt::Formatter;
use std::fmt::Result as FMTResult;
use std::fmt::Display;
use std::path::Path;
use std::result::Result;

use module::todo::TodoModule;

mod todo;

#[derive(Debug)]
pub struct ModuleError {
    desc: String,
}

impl ModuleError {
    fn mk(desc: String) -> ModuleError {
        ModuleError {
            desc: desc,
        }
    }
}

impl Error for ModuleError {

    fn description(&self) -> &str {
        &self.desc[..]
    }

    fn cause(&self) -> Option<&Error> {
        None
    }

}

impl Display for ModuleError {
    fn fmt(&self, f: &mut Formatter) -> FMTResult {
        write!(f, "ModuleError: {}",
               self.description())
    }
}

pub type ModuleResult = Result<(), ModuleError>;

pub trait Module {

    fn new(rt : &Runtime) -> Self;
    fn callnames() -> &'static [&'static str];
    fn name(&self) -> &'static str;

    fn execute(&self, rt : &Runtime) -> ModuleResult;
    fn shutdown(&self, rt : &Runtime) -> ModuleResult;

}


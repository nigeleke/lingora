use std::cell::RefCell;
use std::io::Write;
use std::rc::Rc;

pub type Writer = Rc<RefCell<dyn Write>>;

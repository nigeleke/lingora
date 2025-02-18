use std::{cell::RefCell, io::Write, rc::Rc};

pub type Writer = Rc<RefCell<dyn Write>>;

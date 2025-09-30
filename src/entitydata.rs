use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::tcomponent::TComponent;

pub struct EntityData {
    pub(crate) components: HashMap<&'static str, Rc<RefCell<dyn TComponent>>>,
}

impl EntityData {
    pub fn new() -> Self {
        return Self {
            components: HashMap::new(),
        };
    }
}

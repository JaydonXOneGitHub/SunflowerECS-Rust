use std::{collections::HashMap, rc::Rc};

use boxmut::boxmut::BoxMut;

use crate::tcomponent::TComponent;

pub struct EntityData {
    pub(crate) components: HashMap<&'static str, Rc<BoxMut<Box<dyn TComponent>>>>,
}

impl EntityData {
    pub fn new() -> Self {
        return Self {
            components: HashMap::new(),
        };
    }
}

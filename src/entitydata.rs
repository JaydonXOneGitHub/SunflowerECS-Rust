use std::{any::TypeId, cell::RefCell, collections::HashMap, rc::Rc};

use crate::tcomponent::TComponent;

pub struct EntityData {
    pub(crate) components: HashMap<TypeId, Rc<RefCell<dyn TComponent>>>,
    pub(crate) active: bool,
}

impl EntityData {
    pub fn new() -> Self {
        return Self {
            components: HashMap::new(),
            active: true,
        };
    }
}

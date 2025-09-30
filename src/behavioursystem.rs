use std::{cell::RefCell, rc::Rc};

use crate::{
    tcomponent::TComponent, tdrawsystem::TDrawSystem, tsystem::TSystem,
    tupdatesystem::TUpdateSystem,
};

pub struct BehaviourSystem {
    components: RefCell<Vec<Rc<RefCell<dyn TComponent>>>>,
}

impl BehaviourSystem {
    pub fn new() -> Self {
        return Self {
            components: RefCell::new(Vec::new()),
        };
    }

    pub fn reserve(&mut self, size: usize) -> () {
        if let Result::Ok(mut components) = self.components.try_borrow_mut() {
            components.reserve(size);
        }
    }
}

impl TSystem for BehaviourSystem {
    fn get_drawable(&self) -> Option<&dyn TDrawSystem>
    where
        Self: Sized,
    {
        return Option::Some(self);
    }

    fn get_updatable(&self) -> Option<&dyn TUpdateSystem>
    where
        Self: Sized,
    {
        return Option::Some(self);
    }

    fn on_component_added_to_entity(&self, component: &Rc<RefCell<dyn TComponent>>) -> () {
        if let Result::Ok(mut borrowed) = component.try_borrow_mut() {
            if let Option::Some(_) = borrowed.as_behaviour() {
                if let Result::Ok(mut components) = self.components.try_borrow_mut() {
                    components.push(component.clone());
                }
            }
        }
    }

    fn on_component_removed_from_entity(&self, component: &Rc<RefCell<dyn TComponent>>) -> () {
        if let Result::Ok(mut components) = self.components.try_borrow_mut() {
            components.retain(|c| -> bool {
                return !Rc::ptr_eq(c, &component);
            });
        }
    }
}

impl TDrawSystem for BehaviourSystem {
    fn draw(&self) -> () {
        if let Result::Ok(mut components) = self.components.try_borrow_mut() {
            for c in components.iter_mut() {
                if let Result::Ok(mut comp) = c.try_borrow_mut() {
                    if let Option::Some(bc) = comp.as_behaviour() {
                        bc.draw();
                    }
                }
            }
        }
    }
}

impl TUpdateSystem for BehaviourSystem {
    fn update(&self) -> () {
        if let Result::Ok(mut components) = self.components.try_borrow_mut() {
            for c in components.iter_mut() {
                if let Result::Ok(mut comp) = c.try_borrow_mut() {
                    if let Option::Some(bc) = comp.as_behaviour() {
                        bc.update();
                    }
                }
            }
        }
    }
}

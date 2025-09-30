use std::{cell::RefCell, rc::Rc};

use crate::{
    tcomponent::TComponent, tdrawsystem::TDrawSystem, tsystem::TSystem,
    tupdatesystem::TUpdateSystem,
};

pub struct BehaviourSystem {
    components: Vec<Rc<RefCell<dyn TComponent>>>,
}

impl BehaviourSystem {
    pub fn new() -> Self {
        return Self {
            components: Vec::new(),
        };
    }

    pub fn reserve(&mut self, size: usize) -> () {
        self.components.reserve(size);
    }
}

impl TSystem for BehaviourSystem {
    fn get_drawable(&mut self) -> Option<&mut dyn TDrawSystem>
    where
        Self: Sized,
    {
        return Option::Some(self);
    }

    fn get_updatable(&mut self) -> Option<&mut dyn TUpdateSystem>
    where
        Self: Sized,
    {
        return Option::Some(self);
    }

    fn on_component_added_to_entity(&mut self, component: &Rc<RefCell<dyn TComponent>>) -> () {
        if let Result::Ok(mut borrowed) = component.try_borrow_mut() {
            if let Option::Some(_) = borrowed.as_behaviour() {
                self.components.push(component.clone());
            }
        }
    }

    fn on_component_removed_from_entity(&mut self, component: &Rc<RefCell<dyn TComponent>>) -> () {
        self.components.retain(|c| -> bool {
            return !Rc::ptr_eq(c, &component);
        });
    }
}

impl TDrawSystem for BehaviourSystem {
    fn draw(&mut self) -> () {
        for c in &self.components {
            if let Result::Ok(mut comp) = c.try_borrow_mut() {
                if let Option::Some(bc) = comp.as_behaviour() {
                    bc.draw();
                }
            }
        }
    }
}

impl TUpdateSystem for BehaviourSystem {
    fn update(&mut self) -> () {
        for c in &self.components {
            if let Result::Ok(mut comp) = c.try_borrow_mut() {
                if let Option::Some(bc) = comp.as_behaviour() {
                    bc.update();
                }
            }
        }
    }
}

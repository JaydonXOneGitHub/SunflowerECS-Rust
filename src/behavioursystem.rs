use std::rc::Rc;

use boxmut::boxmut::BoxMut;

use crate::{
    tcomponent::TComponent, tdrawsystem::TDrawSystem, tsystem::TSystem,
    tupdatesystem::TUpdateSystem,
};

pub struct BehaviourSystem {
    components: BoxMut<Vec<Rc<BoxMut<Box<dyn TComponent>>>>>,
}

impl BehaviourSystem {
    pub fn new() -> Self {
        return Self {
            components: BoxMut::new(Vec::new()),
        };
    }

    pub fn reserve(&mut self, size: usize) -> () {
        if let Option::Some(components) = self.components.get_mut() {
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

    fn on_component_added_to_entity(&self, component: &Rc<BoxMut<Box<dyn TComponent>>>) -> () {
        if let Option::Some(borrowed) = component.get_mut() {
            if let Option::Some(_) = borrowed.as_behaviour() {
                if let Option::Some(components) = self.components.get_mut() {
                    components.push(component.clone());
                }
            }
        }
    }

    fn on_component_removed_from_entity(&self, component: &Rc<BoxMut<Box<dyn TComponent>>>) -> () {
        if let Option::Some(components) = self.components.get_mut() {
            components.retain(|c| -> bool {
                return !Rc::ptr_eq(c, &component);
            });
        }
    }
}

impl TDrawSystem for BehaviourSystem {
    fn draw(&self) -> () {
        if let Option::Some(components) = self.components.get_mut() {
            for c in components.iter_mut() {
                if let Option::Some(comp) = c.get_mut() {
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
        if let Option::Some(components) = self.components.get_mut() {
            for c in components.iter_mut() {
                if let Option::Some(comp) = c.get_mut() {
                    if let Option::Some(bc) = comp.as_behaviour() {
                        bc.update();
                    }
                }
            }
        }
    }
}

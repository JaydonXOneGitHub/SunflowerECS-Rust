use std::cell::RefCell;

use crate::{entity::Entity, tbehaviourcomponent::TBehaviourComponent, tcomponent::TComponent};

/// A way to bundle a bunch of components of the same type into one component for the [`Entity`]'s lookup system.
pub struct ComponentCollection<T: TComponent + Sized> {
    components: Vec<RefCell<T>>,
    entity: *mut Entity,
}

impl<T: TComponent> ComponentCollection<T> {
    pub fn new() -> Self {
        return Self {
            components: Vec::new(),
            entity: std::ptr::null_mut(),
        };
    }

    /// Add a component of type T into the collection.
    pub fn add(&mut self, component: T) -> () {
        self.components.push(RefCell::new(component));
    }

    pub fn pop(&mut self) -> Option<RefCell<T>> {
        return self.components.pop();
    }

    pub fn get(&self, index: usize) -> Option<&RefCell<T>> {
        return self.components.get(index);
    }

    pub fn size(&self) -> usize {
        return self.components.len();
    }

    pub fn behaviour_iterate<F>(&mut self, callback: F)
    where
        F: Fn(&mut dyn TBehaviourComponent),
    {
        for c in &mut self.components {
            if let Result::Ok(mut uncelled) = c.try_borrow_mut() {
                if let Option::Some(bc) = uncelled.as_behaviour() {
                    callback(bc);
                }
            }
        }
    }
}

impl<T: TComponent> TComponent for ComponentCollection<T> {}

impl<T: TComponent> TBehaviourComponent for ComponentCollection<T> {
    fn update(&mut self) -> () {
        self.behaviour_iterate(|c| {
            c.update();
        });
    }

    fn draw(&mut self) -> () {
        self.behaviour_iterate(|c| {
            c.draw();
        });
    }

    fn set_entity(&mut self, entity: *mut Entity) -> () {
        self.entity = entity;
    }

    fn get_entity(&self) -> Option<&mut Entity> {
        return unsafe { self.entity.as_mut() };
    }
}

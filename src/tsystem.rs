use std::{cell::RefCell, rc::Rc};

use crate::{tcomponent::TComponent, tdrawsystem::TDrawSystem, tupdatesystem::TUpdateSystem};

pub trait TSystem: 'static {
    /// The method to call for an instance of [`TDrawSystem`].
    fn get_drawable(&mut self) -> Option<&mut dyn TDrawSystem>
// where
    //     Self: Sized,
    {
        return Option::None;
    }

    /// The method to call for an instance of [`TUpdateSystem`].
    fn get_updatable(&mut self) -> Option<&mut dyn TUpdateSystem>
// where
    //     Self: Sized,
    {
        return Option::None;
    }

    fn on_component_added_to_entity(&mut self, _component: &Rc<RefCell<dyn TComponent>>) -> () {}
    fn on_component_removed_from_entity(&mut self, _component: &Rc<RefCell<dyn TComponent>>) -> () {
    }
}

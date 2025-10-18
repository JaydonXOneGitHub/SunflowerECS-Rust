use std::rc::Rc;

use boxmut::boxmut::BoxMut;

use crate::{tcomponent::TComponent, tdrawsystem::TDrawSystem, tupdatesystem::TUpdateSystem};

pub trait TSystem: 'static {
    /// The method to call for an instance of [`TDrawSystem`].
    fn get_drawable(&self) -> Option<&dyn TDrawSystem>
// where
    //     Self: Sized,
    {
        return Option::None;
    }

    /// The method to call for an instance of [`TUpdateSystem`].
    fn get_updatable(&self) -> Option<&dyn TUpdateSystem>
// where
    //     Self: Sized,
    {
        return Option::None;
    }

    /// The hook for when an applied [`Entity`] adds a component.
    fn on_component_added_to_entity(&self, _component: &Rc<BoxMut<Box<dyn TComponent>>>) -> () {}
    /// The hook for when an applied [`Entity`] removes a component.
    fn on_component_removed_from_entity(&self, _component: &Rc<BoxMut<Box<dyn TComponent>>>) -> () {
    }
}

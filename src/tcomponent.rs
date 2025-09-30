pub use std::any::Any;

use crate::tbehaviourcomponent::TBehaviourComponent;

/// The base for all components.
pub trait TComponent: Any {
    /// OVERRIDE FOR TBEHAVIOURCOMPONENT INSTANCES TO ENSURE MAXIMUM COMPATIBILITY!
    fn as_behaviour(&mut self) -> Option<&mut dyn TBehaviourComponent> {
        return Option::None;
    }
}

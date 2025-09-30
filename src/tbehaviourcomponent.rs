use crate::{entity::Entity, tcomponent::TComponent};

/// The base for components which contain logic.
pub trait TBehaviourComponent: TComponent {
    /// The update method.
    fn update(&mut self) -> () {}

    /// The draw method.
    fn draw(&mut self) -> () {}

    /// The method that is called when the attached entity is deleted from the scene.
    fn on_destroyed(&mut self) -> () {}

    /// The method to get the component's attached entity ID.
    fn get_id(&mut self) -> i64 {
        return -1;
    }

    /// Retrieve the assigned entity.
    fn get_entity(&self) -> Option<&mut Entity> {
        return Option::None;
    }

    /// Set which entity this [`TBehaviourComponent`] is assigned to. DON'T CALL THIS YOURSELF!
    fn set_entity(&mut self, _entity: *mut Entity) -> () {}
}

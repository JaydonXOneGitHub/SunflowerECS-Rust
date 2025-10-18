use boxmut::boxmut::BoxMut;

use crate::{entitydata::EntityData, scene::Scene, tcomponent::TComponent};
use std::{
    any::type_name,
    rc::{Rc, Weak},
};

/// The container for many [`TComponent`] or [`TBehaviourComponent`] instances.
pub struct Entity {
    pub(crate) id: i64,
    pub(crate) scene: Option<Weak<BoxMut<Scene>>>,
    pub(crate) entity_data: Option<EntityData>,
}

impl Entity {
    pub fn get_id(&self) -> i64 {
        return self.id;
    }

    pub fn is_valid(&self) -> bool {
        return self.scene.is_some() || self.entity_data.is_some();
    }

    pub fn get_scene(&self) -> Option<Rc<BoxMut<Scene>>> {
        return match &self.scene {
            Option::Some(weak) => weak.upgrade(),
            Option::None => Option::None,
        };
    }

    pub fn destroy(&mut self) -> () {
        if let Option::Some(scene_rc) = self.get_scene() {
            if let Option::Some(scene) = scene_rc.get_mut() {
                scene.destroy_entity(self);
            }
        }
    }

    /// The method to call when you want to add a component.
    pub fn add_component<T>(&mut self, component: T) -> Option<()>
    where
        T: TComponent + 'static,
    {
        let e_ptr = self as *mut Entity;

        if let Option::Some(scene_rc) = self.get_scene() {
            if let Option::Some(scene) = scene_rc.get_ref() {
                let type_id: &'static str = type_name::<T>();

                if let Option::Some(data) = self.entity_data.as_mut() {
                    let boxed: Box<dyn TComponent> = Box::new(component);
                    let componentrc = Rc::new(BoxMut::new(boxed).unwrap());

                    if data
                        .components
                        .insert(type_id, componentrc.clone())
                        .is_none()
                    {
                        let dyncomponentrc: Rc<BoxMut<Box<dyn TComponent>>> = componentrc.clone();

                        if let Option::Some(borrowed) = dyncomponentrc.get_mut() {
                            if let Option::Some(bc) = borrowed.as_behaviour() {
                                bc.set_entity(e_ptr);
                            }
                        }

                        scene.on_component_added_to_entity(&dyncomponentrc);

                        return Option::Some(());
                    }
                }
            }
        }

        return Option::None;
    }

    /// Removes the desired component of type T.
    pub fn remove_component<T>(&mut self) -> Option<()>
    where
        T: TComponent + 'static,
    {
        if let Option::Some(scene_rc) = self.get_scene() {
            if let Option::Some(scene) = scene_rc.get_mut() {
                let type_id: &'static str = type_name::<T>();

                if let Option::Some(data) = self.entity_data.as_mut() {
                    if let Option::Some(component) = data.components.remove(type_id) {
                        if let Option::Some(borrowed) = component.get_mut() {
                            if let Option::Some(bc) = borrowed.as_behaviour() {
                                bc.set_entity(std::ptr::null_mut());
                            }
                        }

                        scene.on_component_removed_from_entity(&component);

                        return Option::Some(());
                    }
                }
            }
        }

        return Option::None;
    }

    pub(crate) fn reset(&mut self) -> () {
        self.entity_data = Option::None;
    }

    /// Allows you to recieve temporary access to a component.
    pub fn use_component<T, F, R>(&mut self, f: F) -> Option<R>
    where
        T: TComponent + 'static,
        F: Fn(&mut T) -> R,
    {
        let type_id: &'static str = type_name::<T>();

        if let Option::Some(data) = self.entity_data.as_mut() {
            if let Option::Some(rc) = data.components.get(type_id) {
                if let Option::Some(borrow) = rc.get_mut() {
                    let any_ref: &mut dyn std::any::Any = &mut *borrow;

                    if let Option::Some(t) = any_ref.downcast_mut::<T>() {
                        return Option::Some(f(t));
                    }
                }
            }
        }

        return Option::None;
    }
}

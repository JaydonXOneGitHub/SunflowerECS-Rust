use crate::{entitydata::EntityData, scene::Scene, tcomponent::TComponent};
use std::{any::TypeId, cell::RefCell, rc::Rc};

/// The container for many [`TComponent`] or [`TBehaviourComponent`] instances.
pub struct Entity {
    pub(crate) id: i64,
    pub(crate) scene: *mut Scene,
}

impl Entity {
    pub fn get_scene(&mut self) -> Option<&mut Scene> {
        unsafe {
            return self.scene.as_mut();
        }
    }

    pub fn get_id(&self) -> i64 {
        return self.id;
    }

    pub fn is_valid(&self) -> bool {
        return !self.scene.is_null();
    }

    pub fn destroy(&mut self) -> () {
        let scene_ptr = self.scene; // copy raw pointer (no borrow)

        if let Option::Some(scene) = unsafe { scene_ptr.as_mut() } {
            scene.destroy_entity(self);
        }
    }

    /// Checks whether or not the entity is active within the [`Scene`].
    pub fn is_active(&self) -> bool {
        let scene_ptr = self.scene; // copy raw pointer (no borrow)

        if let Option::Some(scene) = unsafe { scene_ptr.as_mut() } {
            if let Option::Some(option) = scene.get_entity_data().get(&self.id) {
                if let Option::Some(data) = option {
                    return data.active;
                }
            }
        }

        return false;
    }

    /// The method to call when you want to add a component.
    pub fn add_component<T: TComponent>(&mut self, component: T) -> () {
        let scene_ptr = self.scene;

        if let Option::Some(scene) = unsafe { scene_ptr.as_mut() } {
            let o1: Option<&mut Option<EntityData>> = scene.get_entity_data().get_mut(&self.id);

            if let Option::Some(o2) = o1 {
                let o3: Option<&mut EntityData> = o2.as_mut();

                if let Option::Some(data) = o3 {
                    let type_id: TypeId = TypeId::of::<T>();

                    if !data.components.contains_key(&type_id) {
                        // I can trust that this is safe, as
                        // scene can be accessed around here.
                        let iscene = unsafe { scene_ptr.as_mut().unwrap() };

                        let componentrc = Rc::new(RefCell::new(component));

                        let dyncomponentrc: Rc<RefCell<dyn TComponent>> = componentrc.clone();

                        if let Result::Ok(mut borrowed) = dyncomponentrc.try_borrow_mut() {
                            if let Option::Some(bc) = borrowed.as_behaviour() {
                                bc.set_entity(self as *mut Entity);
                            }
                        }

                        iscene.on_component_added_to_entity(&dyncomponentrc);

                        data.components.insert(type_id, componentrc);
                    }
                }
            }
        }
    }

    /// Removes the desired component of type T.
    pub fn remove_component<T: TComponent>(&mut self) -> Option<()> {
        let scene_ptr = self.scene;

        if let Option::Some(scene) = unsafe { scene_ptr.as_mut() } {
            let o1: Option<&mut Option<EntityData>> = scene.get_entity_data().get_mut(&self.id);

            if let Option::Some(o2) = o1 {
                let o3: Option<&mut EntityData> = o2.as_mut();

                if let Option::Some(data) = o3 {
                    let type_id: TypeId = TypeId::of::<T>();

                    if let Option::Some(component) = data.components.remove(&type_id) {
                        let iscene = unsafe { scene_ptr.as_mut().unwrap() };

                        if let Result::Ok(mut borrowed) = component.try_borrow_mut() {
                            if let Option::Some(bc) = borrowed.as_behaviour() {
                                bc.set_entity(std::ptr::null_mut());
                            }
                        }

                        iscene.on_component_removed_from_entity(&component);

                        return Option::Some(());
                    }
                }
            }
        }

        return Option::None;
    }

    /// Allows you to recieve temporary access to a component.
    pub fn use_component<T, F, R>(&mut self, f: F) -> Option<R>
    where
        T: TComponent + 'static,
        F: Fn(&mut T) -> R,
    {
        let scene_ptr = self.scene;

        let scene = unsafe { scene_ptr.as_mut()? };

        let data = scene.get_entity_data().get(&self.id)?.as_ref()?;
        let rc = data.components.get(&TypeId::of::<T>())?;
        let mut borrow = rc.try_borrow_mut().ok()?;
        let any_ref: &mut dyn std::any::Any = &mut *borrow;
        let t = any_ref.downcast_mut::<T>()?;
        return Option::Some(f(t));
    }
}

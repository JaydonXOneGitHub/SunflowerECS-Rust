use std::{any::TypeId, cell::RefCell, collections::HashMap, rc::Rc, rc::Weak};

use crate::{
    behavioursystem::BehaviourSystem, entity::Entity, entitydata::EntityData,
    tcomponent::TComponent, tsystem::TSystem,
};

/// The struct which controls [`Entity`] and [`TSystem`] instances.
pub struct Scene {
    entity_data: HashMap<i64, Option<EntityData>>,
    entity_objects: HashMap<i64, Rc<RefCell<Entity>>>,
    systems: HashMap<TypeId, Box<dyn TSystem>>,
    id_count: i64,
}

impl Scene {
    pub fn new() -> Self {
        return Self {
            entity_data: HashMap::new(),
            entity_objects: HashMap::new(),
            systems: HashMap::new(),
            id_count: 0,
        };
    }
}

impl Scene {
    /// Creates a new [`Entity`] instance and binds it.
    pub fn create_entity(&mut self) -> Weak<RefCell<Entity>> {
        let scene_ptr: *mut Scene = self as *mut Scene;

        let id: i64 = self.id_count;

        self.entity_data
            .insert(self.id_count, Option::Some(EntityData::new()));

        self.entity_objects.insert(
            self.id_count,
            Rc::new(RefCell::new(Entity {
                id: id,
                scene: scene_ptr,
            })),
        );

        let entity = self.entity_objects.get_mut(&id).unwrap();

        self.id_count += 1;

        return Rc::downgrade(entity);
    }

    /// Update the scene's [`TSystem`] instances.
    pub fn update(&mut self) -> () {
        for sys in self.systems.values_mut() {
            if let Option::Some(updatable) = sys.get_updatable() {
                updatable.update();
            }
        }
    }

    /// Draw the scene's [`TSystem`] instances.
    pub fn draw(&mut self) -> () {
        for sys in self.systems.values_mut() {
            if let Option::Some(drawable) = sys.get_drawable() {
                drawable.draw();
            }
        }
    }

    /// Used for ONLY updating components attached to [`BehaviourSystem`].
    pub fn update_behaviour(&mut self) -> () {
        let btypeid: TypeId = TypeId::of::<BehaviourSystem>();

        if let Option::Some(bs) = self.systems.get_mut(&btypeid) {
            if let Option::Some(updatable) = bs.get_updatable() {
                updatable.update();
            }
        }
    }

    /// Used for ONLY drawing components attached to [`BehaviourSystem`].
    pub fn draw_behaviour(&mut self) -> () {
        let btypeid: TypeId = TypeId::of::<BehaviourSystem>();

        if let Option::Some(bs) = self.systems.get_mut(&btypeid) {
            if let Option::Some(drawable) = bs.get_drawable() {
                drawable.draw();
            }
        }
    }

    pub(crate) fn on_component_added_to_entity(
        &mut self,
        component: &Rc<RefCell<dyn TComponent>>,
    ) -> () {
        for sys in self.systems.values_mut() {
            sys.on_component_added_to_entity(component);
        }
    }

    pub(crate) fn on_component_removed_from_entity(
        &mut self,
        component: &Rc<RefCell<dyn TComponent>>,
    ) -> () {
        for sys in self.systems.values_mut() {
            sys.on_component_removed_from_entity(component);
        }
    }

    fn deinitialize_entity(&mut self, entry: Option<EntityData>, entity: &mut Entity) -> () {
        let values = entry.unwrap().components.into_values();

        for v in values {
            if let Option::Some(mutbc) = v.borrow_mut().as_behaviour() {
                mutbc.on_destroyed();
            }
        }

        self.id_count = entity.id;

        entity.id = -1;
        entity.scene = std::ptr::null_mut();
    }

    pub fn destroy_entity(&mut self, entity: &mut Entity) -> () {
        if let Option::Some(entry) = self.entity_data.remove(&entity.id) {
            self.deinitialize_entity(entry, entity);
        }
    }

    pub fn get_entity_data(&mut self) -> &mut HashMap<i64, Option<EntityData>> {
        return &mut self.entity_data;
    }

    pub fn add_system<T: TSystem>(&mut self, sys: T) -> Option<()> {
        let type_id: TypeId = TypeId::of::<T>();
        let systems: &mut HashMap<TypeId, Box<dyn TSystem>> = &mut self.systems;

        if systems.contains_key(&type_id) {
            return Option::None;
        }

        systems.insert(type_id, Box::new(sys));

        return Option::Some(());
    }

    pub fn remove_system<T: TSystem>(&mut self) -> Option<()> {
        let type_id: TypeId = TypeId::of::<T>();
        let systems: &mut HashMap<TypeId, Box<dyn TSystem>> = &mut self.systems;

        return if systems.remove(&type_id).is_some() {
            Option::Some(())
        } else {
            Option::None
        };
    }
}

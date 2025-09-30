use std::{any::type_name, cell::RefCell, collections::HashMap, rc::Rc};

use crate::{
    behavioursystem::BehaviourSystem, entity::Entity, entitydata::EntityData,
    tcomponent::TComponent, tsystem::TSystem,
};

/// The struct which controls [`Entity`] and [`TSystem`] instances.
pub struct Scene {
    entity_objects: HashMap<i64, Rc<RefCell<Entity>>>,
    systems: HashMap<&'static str, Box<dyn TSystem>>,
    id_count: i64,
}

impl Scene {
    pub fn new() -> Rc<RefCell<Self>> {
        return Rc::new(RefCell::new(Self {
            entity_objects: HashMap::new(),
            systems: HashMap::new(),
            id_count: 0,
        }));
    }
}

impl Scene {
    /// Creates a new [`Entity`] instance and binds it.
    pub fn create_entity(&mut self, scene_rc: &Rc<RefCell<Scene>>) -> Rc<RefCell<Entity>> {
        let id: i64 = self.id_count;

        self.entity_objects.insert(
            self.id_count,
            Rc::new(RefCell::new(Entity {
                id: id,
                scene: Option::Some(Rc::downgrade(scene_rc)),
                entity_data: Option::Some(EntityData::new()),
            })),
        );

        let entity = self.entity_objects.get_mut(&id).unwrap();

        self.id_count += 1;

        return entity.clone();
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
        let btypeid: &'static str = type_name::<BehaviourSystem>();

        if let Option::Some(bs) = self.systems.get_mut(btypeid) {
            if let Option::Some(updatable) = bs.get_updatable() {
                updatable.update();
            }
        }
    }

    /// Used for ONLY drawing components attached to [`BehaviourSystem`].
    pub fn draw_behaviour(&mut self) -> () {
        let btypeid: &'static str = type_name::<BehaviourSystem>();

        if let Option::Some(bs) = self.systems.get_mut(btypeid) {
            if let Option::Some(drawable) = bs.get_drawable() {
                drawable.draw();
            }
        }
    }

    pub(crate) fn on_component_added_to_entity(
        &self,
        component: &Rc<RefCell<dyn TComponent>>,
    ) -> () {
        for sys in self.systems.values() {
            sys.on_component_added_to_entity(component);
        }
    }

    pub(crate) fn on_component_removed_from_entity(
        &self,
        component: &Rc<RefCell<dyn TComponent>>,
    ) -> () {
        for sys in self.systems.values() {
            sys.on_component_removed_from_entity(component);
        }
    }

    fn deinitialize_entity(&mut self, entity: &mut Entity) -> () {
        self.id_count = entity.id;

        entity.id = -1;
        entity.scene = Option::None;

        entity.reset();
    }

    pub fn remove_entity(&mut self, entity: &mut Entity) -> () {
        if !entity.is_valid() {
            return;
        }

        self.id_count = entity.id;

        entity.id = -1;
        entity.scene = Option::None;
    }

    pub fn destroy_entity(&mut self, entity: &mut Entity) -> () {
        self.deinitialize_entity(entity);
    }

    pub fn add_system<T: TSystem>(&mut self, sys: T) -> Option<()> {
        let type_id: &'static str = type_name::<T>();
        let systems: &mut HashMap<&'static str, Box<dyn TSystem>> = &mut self.systems;

        if systems.contains_key(&type_id) {
            return Option::None;
        }

        systems.insert(type_id, Box::new(sys));

        return Option::Some(());
    }

    pub fn remove_system<T: TSystem>(&mut self) -> Option<()> {
        let type_id: &'static str = type_name::<T>();
        let systems: &mut HashMap<&'static str, Box<dyn TSystem>> = &mut self.systems;

        return if systems.remove(&type_id).is_some() {
            Option::Some(())
        } else {
            Option::None
        };
    }
}

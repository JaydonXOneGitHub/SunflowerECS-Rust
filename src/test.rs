use crate::{
    behavioursystem::BehaviourSystem, componentcollection::ComponentCollection, scene::Scene,
    tbehaviourcomponent::TBehaviourComponent, tcomponent::TComponent,
};

pub struct Position {
    pub x: f64,
    pub y: f64,
}

impl TComponent for Position {}

#[test]
fn iterate_scene() -> () {
    let mut scene = Scene::new();

    scene.add_system(BehaviourSystem::new());

    let weak = scene.create_entity();

    if let Option::Some(rc) = weak.upgrade() {
        if let Result::Ok(mut entity) = rc.try_borrow_mut() {
            let mut coll: ComponentCollection<Position> = ComponentCollection::new();

            coll.add(Position { x: 0.0, y: 0.0 });

            assert!(!coll.get_entity().is_some());

            entity.add_component::<ComponentCollection<Position>>(coll);
        }
    }
}

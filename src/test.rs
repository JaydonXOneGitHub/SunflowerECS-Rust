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
    let scene_rc = Scene::new();

    scene_rc
        .get_mut()
        .unwrap()
        .add_system(BehaviourSystem::new());

    let rc = scene_rc.get_mut().unwrap().create_entity(&scene_rc);

    if let Option::Some(entity) = rc.get_mut() {
        let mut coll: ComponentCollection<Position> = ComponentCollection::new();

        coll.add(Position { x: 0.0, y: 0.0 });

        assert!(!coll.get_entity().is_some());

        assert!(
            entity
                .add_component::<ComponentCollection<Position>>(coll)
                .is_some()
        );

        entity.add_component(Position { x: 80.0, y: 63.1 });

        assert!(
            entity
                .use_component(|coll: &mut ComponentCollection<Position>| {
                    coll.add(Position { x: 5.3, y: 23.8 });

                    assert_eq!(coll.size(), 2);
                })
                .is_some()
        );
    }
}

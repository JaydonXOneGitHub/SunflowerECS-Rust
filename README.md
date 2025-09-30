# SunflowerECS-Rust

SunflowerECS-Rust is a hybrid ECS library designed for use in Rust applications.<br>
It's based on my https://github.com/JaydonXOneGitHub/SunflowerECS library, which was made in C#.

# How To Use
Install the library, either by using `cargo add sunflowerecs` or by manually copying the repo and linking to it.

To use:

```rs
use crate::{
    behavioursystem::BehaviourSystem, componentcollection::ComponentCollection, scene::Scene,
    tbehaviourcomponent::TBehaviourComponent, tcomponent::TComponent,
};

pub struct Position {
    pub x: f64,
    pub y: f64,
}

impl TComponent for Position {} // Necessary to be recognized as a valid component

fn main() -> () {
    let scene_rc = Scene::new();

    scene_rc.borrow_mut().add_system(BehaviourSystem::new());

    let rc = scene_rc.borrow_mut().create_entity(&scene_rc);

    if let Result::Ok(mut entity) = rc.try_borrow_mut() {
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
```

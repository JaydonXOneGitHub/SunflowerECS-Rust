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
    let mut scene: Scene = Scene::new();

    scene.add_system(BehaviourSystem::new());

    let weak: Weak<RefCell<Entity>> = scene.create_entity();

    if let Option::Some(rc) = weak.upgrade() {
        if let Result::Ok(mut entity) = rc.try_borrow_mut() {
            let mut coll: ComponentCollection<Position> = ComponentCollection::new();

            coll.add(Position { x: 0.0, y: 0.0 });

            assert!(!coll.get_entity().is_some());

            entity.add_component::<ComponentCollection<Position>>(coll);
        }
    }
}
```

use crate::tsystem::TSystem;

pub trait TUpdateSystem: TSystem {
    fn get_updatable(&self) -> Option<&dyn TUpdateSystem>
    where
        Self: Sized,
    {
        return Option::None;
    }

    fn update(&mut self) -> () {}
}

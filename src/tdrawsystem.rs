use crate::tsystem::TSystem;

pub trait TDrawSystem: TSystem {
    fn get_drawable(&self) -> Option<&dyn TDrawSystem>
    where
        Self: Sized,
    {
        return Option::Some(self);
    }

    fn draw(&mut self) -> () {}
}

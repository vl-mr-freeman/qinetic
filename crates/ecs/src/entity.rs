use crate::world::*;

/// Entity of a [`World`].
pub trait Entity {
    fn begin(world: &World);
    fn end(world: &World);
    fn update(world: &World);
}

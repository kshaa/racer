use bevy::prelude::*;

/// Our physics rollback state container, which will be rolled back and we will
/// use to restore our physics state.
#[derive(Default, Reflect, Hash, Resource, PartialEq, Eq)]
#[reflect(Hash, Resource, PartialEq)]
pub struct RapierRollbackState {
    pub rapier_state: Option<Vec<u8>>,
    pub rapier_checksum: u16,
}

use bevy::{
ecs::{intern::Interned, schedule::NodeConfigs},
prelude::{IntoSystemSetConfigs as _, SystemSet},
};

use crate::markers;
use markers::{Reads, Writes};
/// Constraint for `App::configure_systems` specifying that all writes to T must be executed before
/// the first read
pub fn write_before_read<T: 'static>() -> NodeConfigs<Interned<dyn SystemSet>> {
Writes::<T>::default().before(Reads::<T>::default())
}

/// Constraint for `App::configure_systems` specifying that all reads from T must be executed
/// before the first write
pub fn read_before_write<T: 'static>() -> NodeConfigs<Interned<dyn SystemSet>> {
Reads::<T>::default().before(Writes::<T>::default())
}


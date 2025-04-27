use crate::markers;
use bevy::ecs::schedule::ScheduleConfigs;
use bevy::{
    ecs::intern::Interned,
    prelude::{IntoScheduleConfigs as _, SystemSet},
};
use markers::{Reads, Writes};
/// Constraint for `App::configure_systems` specifying that all writes to T must be executed before
/// the first read
#[must_use]
pub fn write_before_read<T: 'static>() -> ScheduleConfigs<Interned<dyn SystemSet>> {
    Writes::<T>::default().before(Reads::<T>::default())
}

/// Constraint for `App::configure_systems` specifying that all reads from T must be executed
/// before the first write
#[must_use]
pub fn read_before_write<T: 'static>() -> ScheduleConfigs<Interned<dyn SystemSet>> {
    Reads::<T>::default().before(Writes::<T>::default())
}

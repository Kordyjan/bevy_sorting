use std::{
    any::{type_name, TypeId},
    fmt::{self, Debug, Formatter},
    hash::{Hash, Hasher},
    marker::PhantomData,
};

use bevy::{
    ecs::{intern::Interned, label, schedule::NodeConfigs},
    prelude::{IntoSystemConfigs, IntoSystemSetConfigs, System, SystemSet},
};

#[cfg(test)]
mod tests;

pub struct Reads<T: 'static>(PhantomData<fn() -> T>);

impl<T: 'static> Default for Reads<T> {
    fn default() -> Self {
        Self(PhantomData)
    }
}

impl<T> Debug for Reads<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Reads").field(&type_name::<T>()).finish()
    }
}

impl<T> Hash for Reads<T> {
    fn hash<H: Hasher>(&self, _state: &mut H) {}
}

impl<T> Clone for Reads<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Copy for Reads<T> {}

impl<T> PartialEq for Reads<T> {
    fn eq(&self, _other: &Self) -> bool {
        true
    }
}

impl<T> Eq for Reads<T> {}

impl<T> SystemSet for Reads<T> {
    fn dyn_clone(&self) -> Box<dyn SystemSet> {
        Box::new(*self)
    }

    fn as_dyn_eq(&self) -> &dyn label::DynEq {
        self
    }

    fn dyn_hash(&self, mut state: &mut dyn Hasher) {
        TypeId::of::<Self>().hash(&mut state);
    }
}

pub struct Writes<T: 'static>(PhantomData<fn() -> T>);

impl<T: 'static> Default for Writes<T> {
    fn default() -> Self {
        Self(PhantomData)
    }
}

impl<T> Debug for Writes<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Writes").field(&type_name::<T>()).finish()
    }
}

impl<T> Hash for Writes<T> {
    fn hash<H: Hasher>(&self, _state: &mut H) {}
}

impl<T> Clone for Writes<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Copy for Writes<T> {}

impl<T> PartialEq for Writes<T> {
    fn eq(&self, _other: &Self) -> bool {
        true
    }
}

impl<T> Eq for Writes<T> {}

impl<T> SystemSet for Writes<T> {
    fn dyn_clone(&self) -> Box<dyn SystemSet> {
        Box::new(*self)
    }

    fn as_dyn_eq(&self) -> &dyn label::DynEq {
        self
    }

    fn dyn_hash(&self, mut state: &mut dyn Hasher) {
        TypeId::of::<Self>().hash(&mut state);
    }
}

pub trait IntoSystemRW<M>: IntoSystemConfigs<M> {
    fn reads<T: 'static>(self) -> NodeConfigs<Box<dyn System<In = (), Out = ()>>> {
        self.in_set(Reads::<T>::default())
    }

    fn writes<T: 'static>(self) -> NodeConfigs<Box<dyn System<In = (), Out = ()>>> {
        self.in_set(Writes::<T>::default())
    }
}

impl<M, S: IntoSystemConfigs<M>> IntoSystemRW<M> for S {}

pub fn write_before_read<T: 'static>() -> NodeConfigs<Interned<dyn SystemSet>> {
    Writes::<T>::default().before(Reads::<T>::default())
}

pub fn read_before_write<T: 'static>() -> NodeConfigs<Interned<dyn SystemSet>> {
    Reads::<T>::default().before(Writes::<T>::default())
}

use bevy::{
    ecs::schedule::SystemConfigs,
    prelude::{Commands, Event, EventReader, EventWriter, IntoSystemConfigs, SystemParamFunction},
};

use crate::IntoSystemRW;

use impl_trait_for_tuples::impl_for_tuples;

#[cfg(test)]
mod tests;

trait AutoSetArg {
    fn apply(sys: SystemConfigs) -> SystemConfigs;
}

impl<E: Event> AutoSetArg for EventReader<'_, '_, E> {
    fn apply(sys: SystemConfigs) -> SystemConfigs {
        sys.reads::<E>()
    }
}

impl<E: Event> AutoSetArg for EventWriter<'_, E> {
    fn apply(sys: SystemConfigs) -> SystemConfigs {
        sys.writes::<E>()
    }
}

impl AutoSetArg for Commands<'_, '_> {
    fn apply(sys: SystemConfigs) -> SystemConfigs {
        sys
    }
}

#[allow(clippy::let_and_return)]
#[impl_for_tuples(0, 15)]
impl AutoSetArg for Tuple {
    fn apply(sys: SystemConfigs) -> SystemConfigs {
        for_tuples!( #( let sys = <Tuple as AutoSetArg>::apply(sys); )* );
        sys
    }
}

trait InferFlow<Marker> {
    fn in_auto_sets(self) -> SystemConfigs;
}

impl<T, M> InferFlow<M> for T
where
    T::Param: AutoSetArg,
    T: SystemParamFunction<M, In = (), Out = ()>,
    M: 'static,
{
    fn in_auto_sets(self) -> SystemConfigs {
        <T::Param as AutoSetArg>::apply(self.into_configs())
    }
}

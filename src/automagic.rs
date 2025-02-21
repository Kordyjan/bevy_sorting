use bevy::{
    ecs::schedule::SystemConfigs,
    prelude::{Event, EventReader, EventWriter, IntoSystemConfigs, SystemParamFunction},
};

use crate::IntoSystemRW;

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

impl<T: AutoSetArg> AutoSetArg for (T,) {
    fn apply(sys: SystemConfigs) -> SystemConfigs {
        T::apply(sys)
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

//! Library for helping bevy sort systems according to the data flow.
//!
//! # TL;DR
//!
//! ```rust
//! fn finish_quests(awards: Query<&mut Awards, With<Quest>>) {}
//! fn update_equipment(awards: Query<&Awards>, equipment: ResMut<Equipment>) {}
//! fn count_xp(awards: Query<&Awards>, writer: EventWriter<LevelUpEvent>) {}
//! fn update_stats(reader: EventReader<LevelUpEvent>) {}
//! fn run_levelup_animation(reader: EventReader<LevelUpEvent>) {}
//!
//! fn main() {
//!     App::new()
//!         .register_event::<LevelUpEvent>()
//!         .add_systems(
//!             Update,
//!             (
//!                 finish_quests,
//!                 update_equipment,
//!                 count_xp,
//!                 update_stats,
//!                 run_levelup_animation,
//!             ).each_in_auto_sets()
//!         )
//!         .configure_sets(
//!             Update,
//!             (
//!                 write_before_read::<LevelUpEvent>(),
//!                 write_before_read::<Awards>(),
//!             )
//!         )
//!         .run();
//! }
//! ```
//!
//! You don't need to specify system sets or chain systems. Everything will be executed in the intuitive order, with parallelization. First, `finish_quests`, then `count_xp` and later both `update_stats` and `run_levelup_animation` in parallel; `update_equipment` can be run anytime after the end of `finish_quests`, possibly in parallel with other systems.
//!
//! # Want to know more?
//!
//! ## What problem does it solve?
//!
//! Bevy encourages writing small, highly modular systems. They are easier to run in parallel and make code easier to maintain. However, there is a drawback. Without careful grouping, those systems can be executed in unpredictable order. In the best-case scenario, this would lead to annoying one-frame-off event handling; in the worst-case scenario, runtime panics would occur because of the wrong order of resource initialization. Overconstraining things by manually ordering everything is not only error-prone but also can lead to decreased performance due to the lack of parallel execution of systems. This library solves the problem by letting Bevy infer the data flow in systems and specify explicit and verbose constraints that allow the engine to execute systems in predictable order while still maximizing parallelism.
//!
//! ## Isn't it against Bevy's philosophy?
//!
//! I don't think so. This library doesn't change how systems are defined or executed. Both flow inference and constraint specification are done during the app configuration. It doesn't affect code modularity or portability.
//!
//! ## How to use it?
//!
//! Add
//!
//! ```rust
//! use bevy_sorting::prelude::*;
//! ```
//!
//! and you can start using all the library features.
//!
//! ### Flow inference
//!
//! When adding a system to the schedule, you can call `.in_auto_sets()` on it. The signature of the system will be analyzed. Then, the system will be added to appropriate system sets. For example
//!
//! ```rust
//! fn my_system(
//!     resource: ResMut<Something>,
//!     query: Query<&mut Data, With<Marker>>,
//!     events: EventReader<Happening>
//! ) {
//! }
//!
//! let app = App::new().add_systems(Update, my_system.in_auto_sets());
//! ```
//!
//! will create three auto-sets and add `my_system` to them. Those are `Writes<Something>`, `Writes<Data>`, `Reads<Marker>` and `Reads<Happening>`. Any mutable access to a resource or component or access to `EventWriter` is treated as a write. Any immutable access to a resource or component or `EventReader` is treated as a read.
//!
//! It is possible to infer data flow for each system in a tuple, so there is no need for repeated calls of `.in_auto_sets()`:
//!
//! ```rust
//! App::new()
//!     .add_systems(Update,
//!          (
//!             system1,
//!             system2,
//!             system3,
//!         ).each_in_auto_sets()
//!     );
//! ```
//!
//! It is equivalent to
//!
//! ```rust
//! App::new()
//!     .add_systems(Update,
//!          (
//!             system1.in_auto_sets(),
//!             system2.in_auto_sets(),
//!             system3.in_auto_sets(),
//!         )
//!     );
//! ```
//!
//! **Adding systems to auto-sets doesn't do anything to their execution order on its own. The user also needs to specify the constraints.**
//!
//! ### Constraints
//!
//! There are two kinds of constraints:
//! - `read_before_write::<SomeType>()` means that all systems that reads from `SomeType` (ie. is in the `Reads<SomeType>` auto-set) are executed before any system that writes to `SomeType`.
//! - `write_before_read::<SomeType>()` means that all system that writes to `SomeType` will be executed before any system that reads from `SomeType`.
//!
//! You can specify those constraints by calling the `configure_sets` function of `App`. So, continuing the snippet from the previous section, you can write:
//!
//! ```rust
//! app.configure_sets(Update, (write_before_read::<Happening>(), read_before_write::<Marker>()));
//! ```
//!
//! ### Manual flow specification
//!
//! Not everything can be inferred from the system's function signature. Sometimes we use `Commands` to add new resource or component to existing entity. If you want to make constraints besed on those resources or entities you need to mark the data flow manually using `.reads()` and `.writes()` functions.
//!
//! ```rust
//! app.add_systems(Update,
//!     (
//!         system1.in_auto_sets().writes::<NewResource>(),
//!         system2.reads::<NewResource>().writes::<SomeComponent>()
//!     )
//! )
//! ```
//! `system1` will have its normal auto-sets inferred and also will be added to `Writes<NewResource>`. `system2` will be added to `Reads<NewResource>` and `Writes<SomeComponent>` without any auto-sets inferred from the function signature.
//!
//! ### What types can be used in flow markers?
//!
//! Automatic inference creates auto-sets for components, resources, and events. In manual markers, you can use them, but also any rust type (as long as it is `'static`). For example, if you want to group all systems manipulating or reading stats of the player, you can create unit type `PlayerStats` and use it in markers: `.writes::<PlayerStats>` and `.reads::<PlayerStats>`.
//!
//! ## What are the simplest use cases?
//!
//! ### Avoiding one-frame-off event handling
//!
//! ```rust
//! fn count_xp(writer: EventWriter<LevelUpEvent>) {}
//! fn update_stats(reader: EventReader<LevelUpEvent) {}
//! fn run_levelup_animation(reader: EventReader<LevelUpEvent>) {}
//!
//! fn main() {
//!     App::new()
//!         .register_event::<LevelUpEvent>()
//!         .add_systems(Update, (
//!             count_xp,
//!             update_stats,
//!             run_levelup_animation,
//!         ).each_in_auto_sets())
//!         .configure_sets(Update, write_before_read::<LevelUpEvent>())
//!         .run();
//! }
//! ```
//! Each frame `count_xp` will be executed before both `update_stats` and `run_level_up_animation`. The two later may be executed in parallel.
//!
//! ### Correct order of resource initialization
//!
//! ```rust
//! fn read_map_config(mut commands: Commands) {
//!     let config: MapConfig = todo!();
//!     command.insert_resource(config);
//! }
//!
//! fn select_gen_algorithm(
//!     mut commands: Commands,
//!     config: Res<MapConfig>,
//! ) {
//!     let algorithm: WorldGenAlgorithm = todo!();
//!     commands.insert_resource(algorithm);
//! }
//!
//! fn generate_map(
//!     mut commands: Commands,
//!     config: Res<MapConfig>,
//!     algorithm: Res<WorldGenAlgorithm>,
//! ) {
//!     let map: WorldMap = todo!();
//!     commands.insert_resource(map);
//! }
//!
//! fn main() {
//!     App:new()
//!         .add_systems(Startup, (
//!             generate_map.in_auto_sets(),
//!             select_gen_algorithm.in_auto_sets().writes::<WorldGenAlgorithm>,
//!             read_map_config.writes::<MapConfig>(),
//!         ))
//!         .configure_sets(Startup, (
//!             write_before_read::<MapConfig>(),
//!             write_before_read::<WorldGenAlgorithm>(),
//!         ))
//!         .run();
//! }
//! ```
//!
//! There is no risk of panic during the initialization.
//!
//! ## What kind of systems are supported?
//!
//! For now, only systems defined as closures or function references are supported. In the future, I want to support piped systems. Due to Bevy's design, not much more is currently possible.
//!
//! ## Can I mix regular system sets with auto-sets?
//!
//! Of course! There is a small caveat, however. All `in_set()` calls must be called after the `.in_auto_sets()`. Otherwise, you will get a compilation error. This is due to Bevy's design, and I don't think there is anything I can do about it. You can, however, freely mix `.in_set()` calls with any combination of `.writes()` and `.reads()` calls.

mod automagic;
mod markers;
mod ordering;

#[cfg(test)]
mod tests;

pub mod prelude {
    pub use crate::automagic::{InferFlow, InferFlowEach};
    pub use crate::markers::{IntoSystemRW, Reads, Writes};
    pub use crate::ordering::{read_before_write, write_before_read};
}

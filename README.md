# bevy_sorting

Library for creating constraints for ordering bevy system.

## What's this

The crate adds functions `reads` and `writes` that allow specifying how systems access data. The generic parameters for that function can be Components, Resources, Events, or any other Rust type that the user can associate with the notion of reading and writing.

For each type, users can specify if all reads should be executed before the first write or the other way around - the first read should be after all the write operations.

## Example usages

### Assuring correct order of resource initialization

```rust
use bevy::prelude::*;
use bevy_sorting::*;

#[derive(Resurce)]
struct MapConfig(/* data */)

#[derive(Resurce)]
struct WorldGenAlgorithm(/* data */)

#[derive(Resurce)]
struct WorldMap(/* data */)

fn read_map_config(mut commands: Commands) {
    let config: MapConfig = todo!();
    command.insert_resource(config);
}

fn select_gen_algorithm(
    mut commands: Commands, 
    config: Res<MapConfig>,
) {
    let algorithm: WorldGenAlgorithm = todo!();
    commands.insert_resource(algorithm);
}

fn generate_map(
    mut commands: Commands,
    config: Res<MapConfig>,
    algorithm: Res<WorldGenAlgorithm>,
) {
    let map: WorldMap = todo!();
    commands.insert_resource(map);
}

fn main() {
    App:new()
        .add_systems(Startup, (
            generate_map.reads::<MapConfig>().reads::<WorldGenAlgorithm>(),
            select_gen_algorithm.reads::<MapConfig>().writes::<WorldGenAlgorithm>,
            read_map_config.writes::<MapConfig>(),
        ))
        .configure_sets(Startup, (
            write_before_read::<MapConfig>(), 
            write_before_read::<WorldGenAlgorithm>(),
        ))
        .run();
}

```

Systems will be executed in the following order: `[read_map_config, select_gen_algorithm, generate_map]`. Therefore, we can be sure that the app will not panic due to missing resources.

### Assuring that events are read in the same frame as they were emitted

```rust

use bevy::prelude::*;
use bevy_sorting::*;

#[derive(Event)]
struct LevelUpEvent;

fn count_xp(writer: EventWriter<LevelUpEvent>) {
    todo!();
}

fn update_stats(reader: EventReader<LevelUpEvent) {
    todo!();
}

fn run_levelup_animation(reader: EventReader<LevelUpEvent>) {
    todo!();
}

fn main() {
    App::new()
        .register_event::<LevelUpEvent>()
        .add_systems(Update, (
            count_xp.writes::<LevelUpEvent>(),
            update_stats.reads::<LevelUpEvent>(),
            run_levelup_animation.reads::<LevelUpEvent>(),
        ))
        .configure_sets(Update, write_before_read::<LevelUpEvent>())
        .run();
}
```

`count_xp` will be executed before `update_stats` and `run_levelup_animation`. Two later can possibly be executed in parallel. There will be no 1-frame delay between a level-up event being emitted and read.

## It is quite verbose, isn't it?

Yes. This is the first proof of concept. In the future, I plan to add automatic detection of writes and reads based on system signatures.

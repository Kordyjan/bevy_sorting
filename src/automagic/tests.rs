use std::collections::HashMap;

use bevy::prelude::*;

use crate::write_before_read;

use super::InferFlow;

#[test]
fn simple_event_sorting() {
    let mut app = App::new();
    app.add_systems(
        Update,
        (
            reader_only.in_auto_sets(),
            writer_only.in_auto_sets(),
        ),
    )
    .configure_sets(Update, write_before_read::<SomeEvent>());

    panic!(
        "\n\n{:?}\n\n{:?}\n\n{:?}\n\n{:?}\n\n",
        app.get_schedule(Update)
            .unwrap()
            .graph()
            .system_sets()
            .map(|s| (s.0, s.1))
            .filter(|s| s.1.system_type().is_none())
            .collect::<HashMap<_, _>>(),
        app.get_schedule(Update)
            .unwrap()
            .graph()
            .systems()
            .map(|s| (s.0, s.1))
            .collect::<HashMap<_, _>>(),
        app.get_schedule(Update)
            .unwrap()
            .graph()
            .hierarchy()
            .graph(),
        app.get_schedule(Update)
            .unwrap()
            .graph()
            .dependency()
            .graph()
    );
}

#[derive(Resource)]
struct Something;

#[derive(Event)]
struct SomeEvent;

#[derive(Component)]
struct SomeData;

#[derive(Component)]
struct OtherData;

fn reader_only(_reader: EventReader<SomeEvent>) {}

fn writer_only(_writer: EventWriter<SomeEvent>) {}

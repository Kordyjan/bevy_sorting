use std::sync::{Arc, Mutex};

use crate::prelude::*;
use bevy::prelude::*;

#[test]
fn reads_before_writes_test() {
    for _ in 0..8 {
        let trace = Arc::new(Mutex::new(Vec::<String>::new()));

        let mut app = App::new();

        let trace_handle = Arc::clone(&trace);
        app.add_systems(
            Startup,
            (move || {
                trace_handle.lock().unwrap().push("2".to_string());
            })
            .writes::<Marker>(),
        );

        let trace_handle = Arc::clone(&trace);
        app.add_systems(
            Startup,
            (move || {
                trace_handle.lock().unwrap().push("1".to_string());
            })
            .reads::<Marker>(),
        );

        app.configure_sets(Startup, read_before_write::<Marker>());

        app.run();

        assert_eq!(*trace.lock().unwrap(), vec!["1", "2"]);
    }
}
#[test]
fn writes_before_reads_test() {
    for _ in 0..8 {
        let trace = Arc::new(Mutex::new(Vec::<String>::new()));

        let mut app = App::new();

        let trace_handle = Arc::clone(&trace);
        app.add_systems(
            Startup,
            (move || {
                trace_handle.lock().unwrap().push("1".to_string());
            })
            .writes::<Marker>(),
        );

        let trace_handle = Arc::clone(&trace);
        app.add_systems(
            Startup,
            (move || {
                trace_handle.lock().unwrap().push("2".to_string());
            })
            .reads::<Marker>(),
        );

        app.configure_sets(Startup, write_before_read::<Marker>());

        app.run();

        assert_eq!(*trace.lock().unwrap(), vec!["1", "2"]);
    }
}

#[test]
fn with_resources() {
    for _ in 0..8 {
        let mut app = App::new();

        app.add_systems(
            Startup,
            (
                zero_to_res1.writes::<Res1>(),
                res1_to_res2a.reads::<Res1>().writes::<Res2a>(),
                res1_to_res2b.reads::<Res1>().writes::<Res2b>(),
                res2_to_res3.reads::<Res2a>().reads::<Res2b>(),
            ),
        )
        .configure_sets(
            Startup,
            (
                write_before_read::<Res1>(),
                write_before_read::<Res2a>(),
                write_before_read::<Res2b>(),
            ),
        );
        app.run();
    }
}

struct Marker;

#[derive(Resource)]
struct Res1;

#[derive(Resource)]
struct Res2a;

#[derive(Resource)]
struct Res2b;

#[derive(Resource)]
struct Res3;

fn zero_to_res1(mut commands: Commands) {
    commands.insert_resource(Res1);
}

fn res1_to_res2a(mut commands: Commands, _res1: Res<Res1>) {
    commands.insert_resource(Res2a);
}

fn res1_to_res2b(mut commands: Commands, _res1: Res<Res1>) {
    commands.insert_resource(Res2b);
}
fn res2_to_res3(mut commands: Commands, _res2a: Res<Res2a>, _res2b: Res<Res2b>) {
    commands.insert_resource(Res3);
}

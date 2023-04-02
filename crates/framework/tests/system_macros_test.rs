#![allow(dead_code, unused_must_use)]

use starship::{App, Emitter};
use starship_macros::*;

#[derive(Debug, Event)]
struct Event1(String);

#[derive(Debug, State)]
struct State1(usize);

#[derive(Debug, State)]
struct State2(usize);

#[derive(Debug, Resource)]
struct Resource1 {
    pub field: usize,
}

#[derive(Debug, Resource)]
struct Resource2 {
    pub field: usize,
}

// READ

#[system]
async fn read_states(states: StatesRef) {
    dbg!(states);
}

#[system]
async fn read_states_renamed(other: StatesRef) {
    dbg!(other);
}

#[system]
async fn read_state_arg(arg: StateRef<State1>) {
    dbg!(arg);
}

#[system]
async fn read_state_arg_multi(arg1: StateRef<State1>, arg2: StateRef<State2>) {
    dbg!(arg1);
    dbg!(arg2);
}

#[system]
async fn read_state_same_arg(arg1: StateRef<State1>, arg2: StateRef<State1>) {
    dbg!(arg1);
    dbg!(arg2);
}

#[system]
async fn read_resources(resources: ResourcesRef) {
    dbg!(resources);
}

#[system]
async fn read_resources_renamed(other: ResourcesRef) {
    dbg!(other);
}

#[system]
async fn read_resource_arg(arg: ResourceRef<Resource1>) {
    dbg!(arg);
}

#[system]
async fn read_resource_arg_multi(arg1: ResourceRef<Resource1>, arg2: ResourceRef<Resource2>) {
    dbg!(arg1);
    dbg!(arg2);
}

#[system]
async fn read_resource_same_arg(arg1: ResourceRef<Resource1>, arg2: ResourceRef<Resource1>) {
    dbg!(arg1);
    dbg!(arg2);
}

#[system]
async fn read_all_managers(states: StatesRef, resources: ResourcesRef, emitters: EmittersMut) {
    dbg!(states, resources, emitters);
}

// WRITE

#[system]
async fn write_states(states: StatesMut) {
    states.set(State1(123));
}

#[system]
async fn write_states_renamed(other: StatesMut) {
    dbg!(other);
}

#[system]
async fn write_state(arg: StateMut<State1>) {
    **arg = 2;
    dbg!(arg);
}

#[system]
async fn write_resources(resources: ResourcesMut) {
    resources.set(Resource1 { field: 123 });
}

#[system]
async fn write_resources_renamed(other: ResourcesMut) {
    dbg!(other);
}

#[system]
async fn write_resource(arg: ResourceMut<Resource1>) {
    arg.field += 2;
    dbg!(arg);
}

#[system]
async fn write_emitters(emitters: EmittersMut) {
    emitters.set(Emitter::<Event1>::new());
}

#[system]
async fn write_emitters_renamed(other: EmittersMut) {
    dbg!(other);
}

#[system]
async fn write_emitter(em: EmitterMut<Event1>) {
    em.emit(Event1("test".into())).await?;
}

#[system]
async fn write_all_managers(states: StatesMut, resources: ResourcesMut, emitters: EmittersMut) {
    states.set(State1(123));
    resources.set(Resource1 { field: 123 });
    emitters.set(Emitter::<Event1>::new());
}

// MISC

#[system]
fn no_args() {
    dbg!("none");
}

#[system]
fn non_async() {
    dbg!("none");
}

// INVALID

// #[system]
// async fn fail_invalid_return() {
//     return Ok(123);
// }

// TODO?
// #[system]
// async fn fail_invalid_return_type() -> Result<usize> {
//     dbg!("fail");
// }

// #[system]
// async fn fail_self(self) {
//     dbg!(self);
// }

// #[system]
// async fn fail_unknown_type(other: ComponentRef) {
//     dbg!(other);
// }

// #[system]
// async fn fail_unknown_wrapper_type(other: OtherRef<State1>) {
//     dbg!(other);
// }

// #[system]
// async fn fail_manager_with_other_args(manager: StatesRef, arg: StateRef<State1>) {
//     dbg!(manager);
// }

// #[system]
// async fn fail_mut_manager_with_other_args(manager: StatesMut, arg: StateRef<State1>) {
//     manager.add_state(State1(123));
// }

// #[system]
// async fn fail_read_write_arg(arg1: StateRef<State1>, arg2: StateMut<State2>) {
//     dbg!(arg1);
//     **arg2 = 2;
//     dbg!(arg2);
// }

// #[system]
// async fn fail_multi_write_arg(arg1: StateMut<State1>, arg2: StateMut<State2>) {
//     **arg1 = 2;
//     **arg2 = 2;
// }

#[tokio::test]
async fn test_app() {
    let mut app = App::default();
    app.add_initializer(read_states);
    app.add_initializer(read_states_renamed);
    app.add_initializer(read_state_arg);
    app.add_initializer(read_state_arg_multi);
    app.add_initializer(read_state_same_arg);
    app.add_initializer(read_resources);
    app.add_initializer(read_resources_renamed);
    app.add_initializer(read_resource_arg);
    app.add_initializer(read_resource_arg_multi);
    app.add_initializer(read_resource_same_arg);
    app.add_initializer(read_all_managers);
    app.add_initializer(write_states);
    app.add_initializer(write_states_renamed);
    app.add_initializer(write_state);
    app.add_initializer(write_resources);
    app.add_initializer(write_resources_renamed);
    app.add_initializer(write_resource);
    app.add_initializer(write_emitters);
    app.add_initializer(write_emitters_renamed);
    app.add_initializer(write_emitter);
    app.add_initializer(write_all_managers);
    app.add_initializer(non_async);
    app.add_initializer(no_args);
}
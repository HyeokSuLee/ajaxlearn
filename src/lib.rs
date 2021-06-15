use gdnative::prelude::*;

mod processor;
mod systems;
mod structs;

fn start(handle: InitHandle) {
    handle.add_class::<processor::HelloWorld>();
}

godot_init!(start);
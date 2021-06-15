use gdnative::prelude::*;

mod processor;
mod structs;
mod systems;

fn start(handle: InitHandle) {
    handle.add_class::<processor::HelloWorld>();
}

godot_init!(start);

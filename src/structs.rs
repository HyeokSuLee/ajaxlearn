use bevy::prelude::*;
use std::any::{Any, TypeId};
//game state
pub enum GameState {
    Menu,
    Playing,
    Editing,
}

//godot delta time
pub struct Delta {
    pub value: f64,
}

// struct Resource_ON(bool);

//--------------------------------------------------------------------
//                          Components
//--------------------------------------------------------------------

//----------------------------Input-----------------------------------
#[derive(Reflect)]
struct TriggerButton {
    pub next: bool,
    pub value: bool,
    pub treshold: f32,
}
#[derive(Reflect)]
struct InputRaw {
    //
}

struct InputControl {}

enum Part {
    Head,
    LHand,
    RHand,
    LTrigger,
    RTrigger,
    LGrab,
    RGrab,
}

//--------------------------instantiate-------------------------------

//this is component for checking whether it is instantiated or not
pub struct InstantiateProgress {
    pub(crate) next: bool,
    pub value: Entity,
}

//this is component for actual order to instantiate in godot side
#[derive(Copy, Clone)]
pub struct ToInstantiate {
    pub entity: Entity,
    pub source_path: &'static str,
    pub parent_path: &'static str,
    pub name: &'static str,
}

pub struct ListToInstantiate {
    pub list: Vec<ToInstantiate>,
}

//--------------------------movement----------------------------------

pub struct MoveRight {
    pub next: bool,
    pub value: Box<dyn Any>,
}

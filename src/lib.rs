use gdnative::prelude::*;
use bevy::prelude::*;
use lazy_static::*;
use std::sync::Mutex;
use bevy::ecs::component::Components;

#[derive(NativeClass)]
#[inherit(gdnative::prelude::Node)]
struct HelloWorld{
    process: Process, // add this line
}

#[gdnative::methods]
impl HelloWorld {
    fn new(_owner: &gdnative::prelude::Node) -> Self {
        Self {
            process: Process::new()
        }
    }
    
    pub fn _init(_owner: &gdnative::prelude::Node) -> Self {
        Self {
            process: Process::new(), // and this line
        }
    }

    #[export]
    fn _ready(&self, _owner: &gdnative::prelude::Node) {
        godot_print!("hello, world.");
        godot_print!("hello, world.")
    }

    #[export]
    fn _process(&mut self, _owner: &gdnative::prelude::Node,delta: f64) {
        
        self.process.execute(delta);
        
        // godot_print!("hello, world.");
        // godot_print!("hello, world.")
    }

}




fn start(handle: InitHandle) {
    handle.add_class::<HelloWorld>();
}

godot_init!(start);

struct Process {
    app_builder: AppBuilder
}
struct Delta(i32);

impl Process {
    fn new() -> Self {
        
        let mut app_builder = AppBuilder::default();
        app_builder.insert_resource(Delta);
        app_builder.add_startup_system(start_up_system.system());
        app_builder.add_system(normal_system.system());
        
        Process {
           app_builder
        }
    }

    fn execute(&mut self, delta: f64) {                
        // self.resources
        //     .get_mut::<Delta>()
        //     .map(|mut d| d.0 = delta as f32);

       // self.app_builder.set_world(world);
       self.app_builder.app.update();
       // godot_print!("hello, world. -- in Process/execute")
    }
}

lazy_static! {
    static ref WORLD: Mutex<World> = Mutex::new(World::default());
}

// pub fn with_world<F>(mut f: F)
//     where
//         F: FnMut(World),
// {
//     let _result = WORLD.try_lock().map(|mut world| f(world));
// }

// fn app_builder() {
//     App::build()
//         .add_plugins(MinimalPlugins)
//         .add_startup_system(start_up_system.system())
//         .run();
// }
// 
fn start_up_system() {
    godot_print!("hello, world. -- in StartupSystem")
}

fn normal_system() {
    godot_print!("hello, world. -- in NormalSystem")
}
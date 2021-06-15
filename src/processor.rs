use bevy::prelude::*;
use gdnative::prelude::*;

use bevy::reflect::TypeUuid;
use serde::Deserialize;

use crate::structs::*;
use crate::systems::*;

#[derive(NativeClass)]
#[inherit(gdnative::prelude::Spatial)]
pub struct HelloWorld {
    process: Process, // add this line
}

#[gdnative::methods]
impl HelloWorld {
    // 반드시 필수
    fn new(_owner: &gdnative::prelude::Spatial) -> Self {
        Self {
            process: Process::new(),
        }
    }

    // Init은 아무런 의미가 없어 보임.
    // pub fn _init(_owner: &gdnative::prelude::Spatial) -> Self {
    //     Self {
    //         process: Process::new(), // and this line
    //     }
    // }

    #[export]
    fn _ready(&mut self, _owner: &gdnative::prelude::Spatial) {
        godot_print!("hello, world.");

        // let mut world = World::new();
        // self.process.app_builder.run();

        // self.process.app_builder.app.schedule.run(&mut world);
        // if self.process.app_builder.app.world.is_resource_added::<ListToInstantiate>() {
        //     godot_print!("resource added");
        // }

        // let mut to_instantiates = self.process.app_builder.world_mut().get_resource_mut::<ToInstantiate>();
        // let scene = ResourceLoader::godot_singleton().load("res://object.tscn", "PackedScene", false);

        // let scene = load_scene("res://object.tscn");
        // match scene {
        //     Some(_scene) => godot_print!("Loaded child scene successfully!"),
        //     None => godot_print!("Could not load child scene. Check name."),
        // }
        // let scene = if let Some(scene) = scene {
        //     scene
        // } else {
        //     godot_print!("Cannot spawn a child because we couldn't load the template scene");
        //     return;
        // };

        // match instance_scene::<gdnative::prelude::Spatial>(&*scene) {
        //     Ok(spatial) => {
        //         _owner.add_child(spatial.into_shared(), false);
        //
        //         // 단계 나누기 1. 이동해보기, 2. 저장해보기, 3. 저장한것 가져다 이동해보기
        //
        //         //1. move
        //         // unsafe {
        //         //     spatial.set_transform(gdnative::core_types::Transform::translate(Vector3::new(4., 0., 4.)));
        //         // }
        //         // add resource(scene) to bevy
        //         // self.process.app_builder.insert_resource::<>()
        //     }
        //     Err(err) => godot_print!("Could not instance Child : {:?}", err),
        // }
        // instanced_obj = _owner.get_child(0);
        // instanced_obj = match instanced_obj {
        //     Some(x) => x.cast::<gdnative::prelude::Spatial>(),
        //     _ => (),
        // };

        // let instanced_obj = unsafe {
        //     _owner
        //         .get_node_as::<gdnative::prelude::Spatial>("object")
        //         .unwrap()
        // };
        // unsafe {
        //     instanced_obj.set_translation(Vector3::new(
        //         10., 0., 0.,
        //     ));
        // }

        // godot_print!("get OVR");
        // let instanced_obj = unsafe {
        //     _owner
        //         .get_node_as::<gdnative::prelude::Spatial>("test_spatial").unwrap()
        // };
        //
        // let instanced_obj = if let Some(instanced_obj) = instanced_obj {
        //     instanced_obj
        // } else {
        //     godot_print!("Cannot spawn a child because we couldn't load the template instanced_obj");
        //     return;
        // };
        /*
                let instanced_scene = instance_scene::<gdnative::prelude::Spatial>(&scene).unwrap();
                //
                instanced_scene.set_translation(Vector3::new(10., 0., 0.));
                let instanced_scene = unsafe { instanced_scene.into_shared().assume_safe() };
                _owner.add_child(instanced_scene, false);

                let instanced_scene2 = instance_scene::<gdnative::prelude::Spatial>(&scene).unwrap();
                //
                instanced_scene2.set_translation(Vector3::new(-10., 0., 0.));
                let instanced_scene2 = unsafe { instanced_scene2.into_shared().assume_safe() };
                _owner.add_child(instanced_scene2, false);

                instanced_scene2.set_name("object2");


                let instanced_obj = unsafe {
                    _owner
                        .get_node_as::<gdnative::prelude::Spatial>("Spatial")
                        .unwrap()
                };
                godot_print!("before 3 instanced_scene2 name is {}", instanced_scene2.name());

                instanced_obj.set_translation(Vector3::new(0., 10., 0.));

                let instanced_scene3 = instance_scene::<gdnative::prelude::Spatial>(&scene).unwrap();
                godot_print!("I instanced_scene3 name is {}", instanced_scene3.name());
                instanced_scene3.set_translation(Vector3::new(0., 0., 0.));
                godot_print!("T instanced_scene3 name is {}", instanced_scene3.name());

                //베비 리소스에 추가
                // self.process.app_builder.insert_resource(NodesData{ value: _owner});
                // self.process.app_builder.insert_resource(Delta(6));

                // 데이터를 공유 가능하게 하여 이후 add_child에서 instanced_scene3를 move해도 계속 데이터를 사용 가능하다
                let instanced_scene3 = unsafe { instanced_scene3.into_shared().assume_safe() };
                godot_print!("S instanced_scene3 name is {}", instanced_scene3.name());

                _owner.add_child(instanced_scene3, true);

                instanced_scene3.set_name("object3");

                godot_print!("instanced obj name is {}", instanced_obj.name());
                godot_print!("instanced obj filename is {}", instanced_obj.filename());
                godot_print!("instanced obj path is {}", instanced_obj.get_path().to_godot_string());

                godot_print!("instanced_scene1 name is {}", instanced_scene.name());
                godot_print!("instanced_scene1 filename is {}", instanced_scene.filename());
                godot_print!("instanced_scene1 path is {}", instanced_scene.get_path().to_godot_string());

                godot_print!("instanced_scene2 name is {}", instanced_scene2.name());
                godot_print!("instanced_scene2 filename is {}", instanced_scene2.filename());
                godot_print!("instanced_scene2 path is {}", instanced_scene2.get_path().to_godot_string());

                godot_print!("instanced_scene3 name is {}", instanced_scene3.name());
                godot_print!("instanced_scene3 filename is {}", instanced_scene3.filename());
                godot_print!("instanced_scene3 path is {}", instanced_scene3.get_path().to_godot_string());

                let instanced_obj2 = unsafe {
                    _owner
                        .get_node_as::<gdnative::api::directional_light::DirectionalLight>("/root/Spatial/DirectionalLight")
                };
                match instanced_obj2 {
                    Some(x) => godot_print!("Has Directional light"),
                    None => godot_print!("No Directional light"),
                }

                godot_print!("total child count is {}", _owner.get_child_count());

        */
        // let mob_scene: Ref<RigidBody2D, _> = instance_scene(&self.mob);
        // let mob_scene = unsafe { mob_scene.into_shared().assume_safe() };

        // let hud = unsafe { owner.get_node_as_instance::<hud::Hud>("hud").unwrap() };
        // hud.map(|x, o| {
        //     x.update_score(&*o, self.score);
        //     x.show_message(&*o, "Get Ready".into());
        // })
        // .ok()
        // .unwrap_or_else(|| godot_print!("Unable to get hud"));
    }

    #[export]
    fn _process(&mut self, _owner: &gdnative::prelude::Spatial, delta: f64) {
        // 이게 필요 없다. 이미 _ready에서 app_builder.run() 한 것으로 모든 시스템이 반복적으로 돌아간다.
        self.process.execute(delta);

        // let scene = load_scene("res://object.tscn");

        let autoloaded_rust: TRef<gdnative::prelude::Node> =
            unsafe { gdnative::prelude::autoload("Rust").unwrap() };
        // let scene = load_scene("res://object.tscn");

        //Borrow World as mut
        //because cannot borrow self.process.app_builder as mutable more than once at a time
        let mut world = self.process.app_builder.world_mut();

        //Instantiate part>>>
        //Get ListToInstantiate resource
        let mut to_instantiates = world
            .get_resource_mut::<ListToInstantiate>()
            // .map(|Some(x)|x)
            ;

        //List for instantiated Entities
        let mut list_instantated = Vec::<Entity>::new();

        /*match to_instantiates {
            Some(to_instantiates) => godot_print!("length: {}", to_instantiates.list.len()),
            None => (),
        }*/

        // let a = self.process.app_builder.app.world.entities().len();
        // godot_print!("entities count: {}" , a);

        // if to_instantiates.list.len() > 0 {
        //     godot_print!("length: {}", to_instantiates.list.len());
        // }
        // let scene1 = load_scene(r#"res://object.tscn"#).unwrap();
        // godot_print!("after scene 1");

        // let mut list: Vec<ToInstantiate> = Vec::new();
        match to_instantiates {
            Some(mut to_instantiate) => {
                // godot_print!("to instantiate start");
                // godot_print!("length: {}", to_instantiate.list.len());
                // list.clone_from_slice(&to_instantiate.list[..]);
                // list = to_instantiate.list.to_vec();

                for (i, instantiate) in to_instantiate.list.iter().enumerate() {
                    let os = gdnative::api::OS::godot_singleton();
                    godot_print!("instantiate time{} : {}", i, os.get_ticks_msec());
                    //Load Object(Scene)
                    let scene = load_scene(instantiate.source_path).unwrap();

                    //Instantiate
                    let instanced_scene =
                        instance_scene::<gdnative::prelude::Spatial>(&scene).unwrap();
                    instanced_scene.set_global_transform(gdnative::prelude::Transform {
                        basis: Basis::default(),
                        origin: Vector3::new((i * 2) as f32 - 3., 0., 0.),
                    });

                    let instanced_scene = unsafe { instanced_scene.into_shared().assume_safe() };

                    // let parent = _owner.get_node(r#"object_by_ToInstantiate"#).unwrap();
                    //
                    // let parent = unsafe { parent.assume_safe() };
                    _owner.add_child(instanced_scene, true);

                    instanced_scene.set_name(instantiate.name);
                    godot_print!("instanced_scene1 name is {}", instanced_scene.name());
                    godot_print!(
                        "instanced_scene1 filename is {}",
                        instanced_scene.filename()
                    );
                    godot_print!(
                        "instanced_scene1 path is {}",
                        instanced_scene.get_path().to_godot_string()
                    );
                    // unsafe{ Ref::queue_free(instanced_scene.claim().assume_unique());}

                    //Add entity to Instantiated list
                    list_instantated.push(instantiate.entity);
                }
                //Clear All List
                to_instantiate.list.clear();
            }
            None => println!("none"),
        };

        //Change InstantiateProgress State
        for entity in list_instantated {
            let mut instantiate_progress = world.get_mut::<InstantiateProgress>(entity).unwrap();
            instantiate_progress.progress = Progress::Done;
        }

        // godot_print!("1");

        // let one = list.pop().unwrap();
        // godot_print!("source : {}", one.source_path);
        // let scene = load_scene(r#"res:://object.tscn"#);
        // let scene = ResourceLoader::godot_singleton()
        //     .load("res:://object.tscn", "PackedScene", false)
        //     .unwrap();

        // godot_print!("2");
        // let scene = unsafe { scene.assume_thread_local() };
        // let scene = scene.cast::<PackedScene>();
        // let scene = scene.unwrap();

        /*
                let instanced_scene3 = unsafe {
                    _owner
                        .get_node_as::<gdnative::prelude::Spatial>("/root/Spatial/object3")
                        .unwrap()
                };

                instanced_scene3.set_translation(instanced_scene3.translation() + Vector3::new(-0.1, 0., 0.));

                let instanced_scene2 = unsafe {
                    _owner
                        .get_node_as::<gdnative::prelude::Spatial>("/root/Spatial/object2")
                        .unwrap()
                };
                instanced_scene2.set_translation(instanced_scene2.translation() + Vector3::new(0.1, 0., 0.));

                godot_print!("@@total child count is {}", _owner.get_child_count());
                godot_print!("@@owner name is {}", _owner.name());
                godot_print!("@@owner path is {}", _owner.get_path().to_godot_string());
        */
        // godot_print!("hello, world.");
        // godot_print!("hello, world.")
        godot_print!(
            "{}",
            gdnative::api::Performance::godot_singleton()
                .get_monitor(gdnative::api::Performance::TIME_FPS)
        );
    }
    #[export]
    fn _physics_process(&mut self, _owner: &gdnative::prelude::Spatial, delta: f64) {}
}

//--------------------------------------------------------------------------------------------------
// - Load and Instantiate scene
//--------------------------------------------------------------------------------------------------

pub fn load_scene(path: &str) -> Option<Ref<PackedScene, ThreadLocal>> {
    let scene = ResourceLoader::godot_singleton().load(path, "PackedScene", false)?;

    let scene = unsafe { scene.assume_thread_local() };

    scene.cast::<PackedScene>()
}

/// Root here is needs to be the same type (or a parent type) of the node that you put in the child
///   scene as the root. For instance Spatial is used for this example.
fn instance_scene<Root>(scene: &PackedScene) -> Result<Ref<Root, Unique>, ManageErrs>
where
    Root: gdnative::GodotObject<RefKind = ManuallyManaged> + SubClass<gdnative::prelude::Node>,
{
    let instance = scene
        .instance(PackedScene::GEN_EDIT_STATE_DISABLED)
        .ok_or(ManageErrs::CouldNotMakeInstance)?;
    let instance = unsafe { instance.assume_unique() };

    instance
        .try_cast::<Root>()
        .map_err(|instance| ManageErrs::RootClassNotSpatial(instance.name().to_string()))
}

#[derive(Debug, Clone, PartialEq)]
pub enum ManageErrs {
    CouldNotMakeInstance,
    RootClassNotSpatial(String),
}

//--------------------------------------------------------------------------------------------------
// Bevy part
//--------------------------------------------------------------------------------------------------

// #[derive(Debug, Clone, Eq, PartialEq, Hash)]
// enum GodotState {
//     New,
//     Ready,
//     Process,
// }

struct Process {
    app_builder: AppBuilder,
}

impl Process {
    fn new() -> Self {
        let mut app_builder = AppBuilder::default();
        app_builder.add_plugins(MinimalPlugins);

        app_builder.insert_resource(Delta { value: 0. });

        let os = gdnative::api::OS::godot_singleton();
        godot_print!("order instantiate time1,2 : {}", os.get_ticks_msec());

        app_builder.insert_resource(ListToInstantiate { list: Vec::new() });
        // app_builder.add_state(GodotState::New);
        // app_builder.add_asset::<ToInstantiate>();1
        // app_builder.insert_resource(Resource_ON(false));
        app_builder.add_startup_system(start_up_system.system());
        // app_builder.add_system(normal_system.system());
        // app_builder.add_system(system2.system());

        //node base system
        app_builder.add_startup_system(setup_lerp.system());
        app_builder.add_system(calc_system.system());

        Process { app_builder }
    }

    fn add_resource(&mut self) {}

    fn execute(&mut self, delta: f64) {
        let mut resource_delta = self
            .app_builder
            .world_mut()
            .get_resource_mut::<Delta>()
            .unwrap();
        resource_delta.value = delta;

        // self.app_builder.set_world(world);
        self.app_builder.app.update();
        // godot_print!("hello, world. -- in Process/execute")
    }
}

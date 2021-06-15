use bevy::prelude::*;
use gdnative::prelude::*;

use crate::structs::*;
use bevy::ecs::component::Component;

pub fn start_up_system(mut command: Commands, mut to_instantiate: ResMut<ListToInstantiate>) {
    godot_print!("hello, world. -- in StartupSystem");
    let os = gdnative::api::OS::godot_singleton();
    godot_print!("order instantiate time3 : {}", os.get_ticks_msec());
    // to_instantiate.list.push(ToInstantiate {
    //     source_path: "res://object.tscn",
    //     parent_path: "/root/Rust",
    //     name: "object_by_ToInstantiate3",
    // });

    call_spawn(
        &mut command,
        "res://object.tscn",
        "/root/Rust",
        "new_object_by_function_callspawn",
        &mut to_instantiate,
    );

    /*//Instantiate performance test
    for i in 0..100 {
        to_instantiate.list.push(ToInstantiate {
            source_path: "res://object.tscn",
            parent_path: "/root/Rust",
            name: "object_by_ToInstantiate3",
        });
    }*/
    // let scene = ResourceLoader::godot_singleton().load("res://object.tscn", "PackedScene", false);

    // let scene = ResourceLoader::godot_singleton().load("res://object.tscn", "PackedScene", false);
    // godot_print!("hello, world. -- in NormalSystem")
    // let scene = ResourceLoader::godot_singleton().load("res://object.tscn", "PackedScene", false);

    // let scene = load_scene("res://object.tscn").unwrap();

    // let instanced_scene3 = unsafe {
    //         get_node_as::<gdnative::prelude::Spatial>("/root/Spatial/object3")
    //         .unwrap()
    // };
    // let instanced_scene = instance_scene::<gdnative::prelude::Spatial>(&scene).unwrap();
    // //
    // instanced_scene.set_translation(Vector3::new(10., 0., 0.));
    // let instanced_scene = unsafe { instanced_scene.into_shared().assume_safe() };

    //오토로드를 이용해 베비에서 노드를 직접 가져와 컨트롤 할 수 있다. 이 말은 노드를 굳이 베비 리소스로 저장할 필요가 없다는 의미
    // let autoloaded_rust:TRef<gdnative::prelude::Node> =
    //     unsafe {
    //         gdnative::prelude::autoload("Rust").unwrap()
    //     };

    // godot_print!("autoloaded_rust path {}",  autoloaded_rust.get_path().to_godot_string());
    //
    // autoloaded_rust.add_child(instanced_scene, true);
    // instanced_scene.set_name("rust_child_object");
}

pub fn normal_system(
    // mut res_on: ResMut<Resource_ON>,
    delta: Res<Delta>,
    mut to_instantiate: ResMut<ListToInstantiate>,
    mut query: Query<(Entity, &InstantiateProgress)>,
) {
    godot_print!("---------------------------------");
    godot_print!("hello, world. -- in NormalSystem1");

    // to_instantiate.list.push(ToInstantiate {
    //     source_path: "res:://object.tscn",
    //     parent_path: "/root/Rust",
    //     name: "object_by_ToInstantiate",
    // });

    // if res_on.0 == false {
    //     godot_print!("hello, world. -- in NormalSystem2");
    //
    //     res_on.0 = true;
    // }

    let autoloaded_rust: TRef<gdnative::prelude::Node> =
        unsafe { gdnative::prelude::autoload("Rust").unwrap() };

    let delta_f32 = delta.value.clone() as f32;

    /*//Controll node's transform manually
    query.for_each_mut(|(e, i)| match i.0 {
        Progress::Done => {
            unsafe {
                autoloaded_rust
                    .as_ref()
                    .get_node_as::<gdnative::prelude::Spatial>(
                        "/root/Rust/new_object_by_function_callspawn",
                    )
                    .map_or(None, |instance| {
                        instance.set_translation(
                            instance.translation() + Vector3::new(100., 0., 0.) * delta_f32,
                        );
                        Some(instance)
                    })
            };
        }
        _ => (),
    });*/

    // let instanced_scene3 = unsafe {
    //     autoloaded_rust
    //         .as_ref()
    //         .get_node_as::<gdnative::prelude::Spatial>(
    //             "/root/Rust/new_object_by_function_callspawn",
    //         )
    //         .map_or(None, |instance| {
    //             instance.set_translation(
    //                 instance.translation() + Vector3::new(0.05, 0., 0.) * time.delta_seconds(),
    //             );
    //             Some(instance)
    //         })
    // };

    // instanced_scene3.set_translation(instanced_scene3.translation() + Vector3::new(-0.1, 0., 0.));

    // _owner.add_child(instanced_scene, false);
    //
    // let instanced_obj = unsafe {
    //
    //     get_node_as::<gdnative::prelude::Spatial>("Spatial")
    //         .unwrap()
    // };

    // let instanced_scene3 = unsafe {
    //     autoloaded_rust.as_ref()
    //         .get_node_as::<gdnative::prelude::Spatial>("/root/Rust/rust_child_object")
    //         .unwrap()
    // };
    //
    // instanced_scene3.set_translation(instanced_scene3.translation() + Vector3::new(-0.1, 0., 0.));

    //
    // godot_print!("@@autoloaded_rust name is {}", autoloaded_rust.name());
    // godot_print!("@@autoloaded_rust path is {}", autoloaded_rust.get_path().to_godot_string());
    // godot_print!("@@total child count is {}", autoloaded_rust.get_child_count());
}

pub fn system2() {
    godot_print!("hello, world. -- in NormalSystem2");
    godot_print!("---------------------------------");
}

//오브젝트 만들고 포지션 가져오기

//오브젝트를 특정하는 요소. 엔티티
//오브젝트에 특정 기능을 부여하는 요소. 컴포넌트
//기능을 수행하는 요소 시스템
// 결로 특정 오브젝트에 기능을 수행 >
// 1. 엔티티에 컴포넌트를 부여함.
// 2. 시스템이 특정 컴포넌트에 대해 기능을 수행함.
// 추가
// 시스템이 특정 컴포넌트를 가진 엔티티가 생성 되었을떼에 비로소 기능을 수행함. >
// 1. 시스템1에서 생성함수 호출( 베비에서 생성 주문. 엔티티 정보를 리턴. 고돗 생성리스트에 엔티티 정보 추가.)
// 2. 시스템1에서 리턴된 엔티티에 컴포넌트1을 추가
// 3. 고돗에서 생성리스트에 대해 생성 수행 후 엔티티 생성 상태 변경.
// 4. 시스템2(기능수행)에서 (엔티티, 컴포넌트1) 번들 쿼리.
// 5. 시스템2에서 쿼리된 엔티티의 생성 상태 조회 후, 생성 완료면 기능 수행.
//
// -트리거 입력시 총알 생성 / () : 번들 / [] : 시스템
// 일반
// [트리거 버튼 ON] > [ (리소스 패스, 엔티티, ToInstantiate) 생성예약 > (AddForce) 추가] > [AddForcing 수행]
//
// 커넥터 방식
// [트리거 버튼ON] > 노드 커넥터 (bool) > [(리소스 패스, 엔티티, ToInstantiate) 생성예약] > 노드 커넥터(entity)
// > [(AddForce) 붙이기] > 고돗 생성 >
//
// 트리거 입력시 총알 생성 되게 에디터로 제작 - 수동으로 컴포넌트를 붙여줌.
// -.각 시스템끼리 선을 이으면 자동으로 해당 엔티티에 필요한 컴포넌트를 붙여줌.
// -.혹은 엔티티에 실행할 시스템들을 리스트로 가지는 컴포넌트를 붙임. 한 시스템이 (SystemList) 컴포넌트의 리스트의 각 시스템의 단계 이동 및 컴포넌트 Attach/Del을 수행
// ->시스템으로 직접 에디팅. 하지만 병렬 수행 가능으로 봄.
// [트리거버튼 ON] 블록 놓기 > [(리소스 패스, 엔티티, ToInstantiate) 생성예약] 블록 놓기 > 블록에서 선 뽑아내어 entity 받아옴 >
// 받아온 엔티티에 (AddForce) 붙여줌 >

//걸어놓은 특정 조건을 만족하면, 그때 해당 컴포넌트를 엔티티에 붙여 맞는 시스템이 작동하도록 함
struct ToBeAddComponent {
    component: dyn Component,
}
//걸어놓은 특정 조건을 만족하면, 그때 해당 컴포넌트를 엔티티에서 제거하여 시스템이 중지하도록 함
struct ToBeDelComponent {
    component: dyn Component,
}

fn system_addcomponent() {}

fn system_delcomponent() {}

//IF 문
//1. 커넥터 타입
// : fn system_if(compare_arg: Enum_Compare, target_a, target_b, if_component){
//     match compare_arg{
//      greater =>
//      smaller =>
//      equal =>
//     }
// }

//2. 직접 컴포넌트 에디팅 타입
//3. 수행 시스템 리스트를 통한 시스템 제어 타입

fn call_spawn(
    mut commands: &mut Commands,
    source_path: &'static str,
    parent_path: &'static str,
    name: &'static str,
    mut list_to_instantiate: &mut ResMut<ListToInstantiate>,
) -> Entity {
    let entity = commands.spawn().id();
    commands.insert_one(InstantiateProgress {
        next: false,
        value: entity,
    });

    list_to_instantiate.list.push(ToInstantiate {
        entity,
        source_path,
        parent_path,
        name,
    });

    entity
}

fn call_do_instantiated_entity(mut query: &Query<(Entity, &InstantiateProgress)>) {}

fn test() {
    let mut registry = TypeRegistry::default();
    registry.register::<u32>();
    registry.register::<isize>();
    registry.register::<usize>();
    registry.register::<Bar>();
    registry.register::<String>();
    registry.register::<i8>();
    registry.register::<i32>();

    let serializer = ReflectSerializer::new(&foo, &registry);
}

use bevy::prelude::*;
use gdnative::prelude::*;

use crate::structs::*;
use bevy::utils::{HashMap, Uuid};
use std::any::{Any, TypeId};

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

fn call_spawn(
    mut commands: &mut Commands,
    source_path: &'static str,
    parent_path: &'static str,
    name: &'static str,
    mut list_to_instantiate: &mut ResMut<ListToInstantiate>,
) -> Entity {
    let entity = commands.spawn().id();

    commands.entity(entity).insert(InstantiateProgress {
        progress: Progress::Stanby,
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

/* fn test() {
    let mut registry = TypeRegistry::default();
    registry.register::<u32>();
    registry.register::<isize>();
    registry.register::<usize>();
    registry.register::<Bar>();
    registry.register::<String>();
    registry.register::<i8>();
    registry.register::<i32>();

    let serializer = ReflectSerializer::new(&foo, &registry);
} */

//-------------------------------------node base system----------------------------------------------
use crate::{calc, calc_job, check_and_get_value, make_tag};
use paste::paste;
pub struct OutCheck {
    node_done: bool,
}

/// put specific value and input value as much you want. if it has output value, then just insert '_'.
/// Example (has output): make_tag!(Calc, [SPEC] operation: ArithmeticOperation , [IN] a, [IN] b, [OUT] _);
/// Example (no output): make_tag!(Calc, [SPEC] operation: ArithmeticOperation , [IN] a, [IN] b);
/// Example (no output, no spec): make_tag!(Calc, , [IN] a, [IN] b);
#[macro_export]
macro_rules! make_tag {
    ($struct_name:ident, $([SPEC] $tag_specific_value: ident : $ty:ty),* ,  $([IN] $input_abc: ident),*, $([OUT] $has_output: tt),*) => {
         paste! {
            pub struct $struct_name {
            input_type:Option<TypeId>,
            $($tag_specific_value: $ty,)*

                $(
                    [<input_ $input_abc _entity>]: Option<Entity>,
                    [<input_ $input_abc _outcheck_entity>]: Option<Entity>,
                )*
                $(
                    [<output $has_output entity>] :Option<Entity>,
                    [<output $has_output outcheck_entity>]  : Option<Entity>,
                )*
            }
        }
    }
}

make_tag!(Calc, [SPEC] operation: ArithmeticOperation , [IN] a, [IN] b, [OUT] _);
make_tag!(Copy, , [IN] a, [OUT]_);
make_tag!(Print, , [IN] a,);

// struct Calc {
//     operation: ArithmeticOperation,
//     input_type: Option<TypeId>,
//     input_a_entity: Option<Entity>,
//     input_a_outcheck_entity: Option<Entity>,
//     input_b_entity: Option<Entity>,
//     input_b_outcheck_entity: Option<Entity>,
//     output_entity: Option<Entity>, // no type cause it is same with input
//     output_outcheck_entity: Option<Entity>, // no type cause it is same with input
// }
//TODO: Move를 만들지 말지 나중에 결정. 포인터와 같이 되어, 나중에 null포인터와 같은 오류가 생길 가능성이 있음.
//      물론 에디터로 방지해도 되지만, 굳이 move가 필요한가 고민. move를 과연 어디에 쓸 것인가.
#[derive(Debug)]
struct Move {
    input_type: Option<TypeId>,
    input_a_entity: Option<Entity>,
    input_a_outcheck_entity: Option<Entity>,
    output_entity: Option<Entity>,
    output_outcheck_entity: Option<Entity>, // no type cause it is same with input
}
#[derive(Debug)]
struct Make {
    input_type: Option<TypeId>,
    input_a_entity: Option<Entity>,
    input_a_outcheck_entity: Option<Entity>,
    output_entity: Option<Entity>,
    output_outcheck_entity: Option<Entity>, // no type cause it is same with input
}
// struct Copy {
//     input_type: Option<TypeId>,
//     input_a_entity: Option<Entity>,
//     input_a_outcheck_entity: Option<Entity>,
//     output_entity: Option<Entity>,
//     output_outcheck_entity: Option<Entity>, // no type cause it is same with input
// }
// struct Print {
//     input_type: Option<TypeId>,
//     input_a_entity: Option<Entity>,
//     input_a_outcheck_entity: Option<Entity>,
// }

//TODO ------------------------------<<<<<<

//글로벌 데이터는 오브젝트 간에.
//로컬 데이터는 엔티티 내부
//TODO: 전반적으로 사칙연산 방식으로 사용함에 따른 최적화가 필요. 군더더기가 너무 많다.
//TODO: 사칙연산 방식에서 데이터 밸류는 1개 값만 가지므로, 해쉬맵을 쓸 이유가 없음. 단일 값으로 바꾸기.
//TODO: 전부 Calc 태그를 사용하므로 아웃풋 밸류 중에 타입id가 필요 없다.
//TODO: 병목 현상 발생시에 방안 1. calc_system을 parrarell하게 바꾸기. 2. calc_system을 4개의 시스템으로 쪼개기( add, sub, mul, div)
//TODO: 기초 시스템
// - 사칙연산
// - 블록(노드 연결구조) 로드, 세이브
// - 데이터 할당, 이동, 복사, 삭제
// - 해쉬노드/벡터노드 , 가져오기, 입력하기
//TODO: 매크로로 중복되는 부분들 모듈화. 코드에 표식 넣음.
//TODO: datas 말고 단일 데이터로 만들기
//TODO: remove_completed_nodes_system 만들기. 이유 : 메모리를 무한대로 잡아먹을 우려가있다.

#[derive(Default)]
pub struct Datas {
    bool: Option<bool>,
    i32: Option<i32>,
    f32: Option<f32>,
    // vector3: Option<Vec3>,
    // string: Option<String>,
}

enum ArithmeticOperation {
    ADD,
    SUBTRACT,
    MULTIPLY,
    DIVIDE,
}

pub fn setup_lerp(mut commands: Commands) {
    for i in 0..30000 {
        //---------------------------------------
        //Create Node UUID **node == job
        //---------------------------------------
        //make node
        let id_node_add = Uuid::new_v4();
        let id_node_subtract = Uuid::new_v4();
        let id_node_multiply = Uuid::new_v4();
        let id_node_copy = Uuid::new_v4();
        let id_node_print = Uuid::new_v4();

        //make data bank slot
        let id_value_a = Uuid::new_v4();
        let id_value_b = Uuid::new_v4();
        let id_value_t = Uuid::new_v4();
        let id_value_subtract_out = Uuid::new_v4();
        let id_value_add_out = Uuid::new_v4();
        let id_value_multiply_out = Uuid::new_v4();
        let id_value_copied_out = Uuid::new_v4();
        //---------------------------------------
        //Create Connector
        //---------------------------------------
        /*

         a      a      t
          \      \      \
        b -- sub -- add -- mul

        */
        let data_value_a = Some(0.);
        let data_value_b = Some(100.);
        let data_value_t = Some(0.4);
        let data_value_subtract_out = None;
        let data_value_add_out = None;
        let data_value_multiply_out = None;
        let data_value_copied_out = None;
        //---------------------------------------
        //Create Entity ( value )
        //---------------------------------------
        //Values
        let entity_value_a = commands
            .spawn()
            .insert(Datas {
                f32: data_value_a,
                ..Default::default()
            })
            .id();
        let entity_value_a_outcheck = commands.spawn().insert(OutCheck { node_done: true }).id();

        let entity_value_b = commands
            .spawn()
            .insert(Datas {
                f32: data_value_b,
                ..Default::default()
            })
            .id();
        let entity_value_b_outcheck = commands.spawn().insert(OutCheck { node_done: true }).id();

        let entity_value_t = commands
            .spawn()
            .insert(Datas {
                f32: data_value_t,
                ..Default::default()
            })
            .id();
        let entity_value_t_outcheck = commands.spawn().insert(OutCheck { node_done: true }).id();

        let entity_value_add_out = commands
            .spawn()
            .insert(Datas {
                f32: data_value_add_out,
                ..Default::default()
            })
            .id();
        let entity_value_add_out_outcheck =
            commands.spawn().insert(OutCheck { node_done: false }).id();

        let entity_value_subtract_out = commands
            .spawn()
            .insert(Datas {
                f32: data_value_subtract_out,
                ..Default::default()
            })
            .id();
        let entity_value_subtract_out_outcheck =
            commands.spawn().insert(OutCheck { node_done: false }).id();

        let entity_value_multiply_out = commands
            .spawn()
            .insert(Datas {
                f32: data_value_multiply_out,
                ..Default::default()
            })
            .id();
        let entity_value_multiply_out_outcheck =
            commands.spawn().insert(OutCheck { node_done: false }).id();

        let entity_value_copied_out = commands
            .spawn()
            .insert(Datas {
                f32: data_value_copied_out,
                ..Default::default()
            })
            .id();
        let entity_value_copied_out_outcheck =
            commands.spawn().insert(OutCheck { node_done: false }).id();

        //---------------------------------------
        //Add input, output data ids to connector
        //---------------------------------------
        let mut calc_add = Calc {
            operation: ArithmeticOperation::ADD,
            input_type: None,
            input_a_entity: None,
            input_a_outcheck_entity: None,
            input_b_entity: None,
            input_b_outcheck_entity: None,
            output_entity: None,
            output_outcheck_entity: None,
        };

        let mut calc_sub = Calc {
            operation: ArithmeticOperation::SUBTRACT,
            input_type: None,
            input_a_entity: None,
            input_a_outcheck_entity: None,
            input_b_entity: None,
            input_b_outcheck_entity: None,
            output_entity: None,
            output_outcheck_entity: None,
        };

        let mut calc_mul = Calc {
            operation: ArithmeticOperation::MULTIPLY,
            input_type: None,
            input_a_entity: None,
            input_a_outcheck_entity: None,
            input_b_entity: None,
            input_b_outcheck_entity: None,
            output_entity: None,
            output_outcheck_entity: None,
        };

        let mut copy = Copy {
            input_type: None,
            input_a_entity: None,
            input_a_outcheck_entity: None,
            output_entity: None,
            output_outcheck_entity: None,
        };

        let mut print = Print {
            input_type: None,
            input_a_entity: None,
            input_a_outcheck_entity: None,
        };

        //Inputs_type
        calc_add.input_type = Some(TypeId::of::<f32>());
        calc_sub.input_type = Some(TypeId::of::<f32>());
        calc_mul.input_type = Some(TypeId::of::<f32>());
        copy.input_type = Some(TypeId::of::<f32>());
        print.input_type = Some(TypeId::of::<f32>());

        //Inputs_entity
        calc_add.input_a_entity = Some(entity_value_a);
        calc_add.input_a_outcheck_entity = Some(entity_value_a_outcheck);
        calc_add.input_b_entity = Some(entity_value_subtract_out);
        calc_add.input_b_outcheck_entity = Some(entity_value_subtract_out_outcheck);
        calc_sub.input_a_entity = Some(entity_value_b);
        calc_sub.input_a_outcheck_entity = Some(entity_value_b_outcheck);
        calc_sub.input_b_entity = Some(entity_value_a);
        calc_sub.input_b_outcheck_entity = Some(entity_value_a_outcheck);
        calc_mul.input_a_entity = Some(entity_value_add_out);
        calc_mul.input_a_outcheck_entity = Some(entity_value_add_out_outcheck);
        calc_mul.input_b_entity = Some(entity_value_t);
        calc_mul.input_b_outcheck_entity = Some(entity_value_t_outcheck);
        copy.input_a_entity = Some(entity_value_multiply_out);
        copy.input_a_outcheck_entity = Some(entity_value_multiply_out_outcheck);
        print.input_a_entity = Some(entity_value_copied_out);
        print.input_a_outcheck_entity = Some(entity_value_copied_out_outcheck);
        //Ouputs
        calc_add.output_entity = Some(entity_value_add_out);
        calc_add.output_outcheck_entity = Some(entity_value_add_out_outcheck);
        calc_sub.output_entity = Some(entity_value_subtract_out);
        calc_sub.output_outcheck_entity = Some(entity_value_subtract_out_outcheck);
        calc_mul.output_entity = Some(entity_value_multiply_out);
        calc_mul.output_outcheck_entity = Some(entity_value_multiply_out_outcheck);
        copy.output_entity = Some(entity_value_copied_out);
        copy.output_outcheck_entity = Some(entity_value_copied_out_outcheck);

        //---------------------------------------
        //Create Node( job )-Connectors Entity
        //---------------------------------------
        // think that calc{ Add } and Calc{ Sub } is diffrent tag. This concept is temporarly
        commands.spawn().insert(calc_add);
        commands.spawn().insert(calc_sub);
        commands.spawn().insert(calc_mul);
        commands.spawn().insert(copy);
        commands.spawn().insert(print);
    }
}

pub fn calc_system(
    mut commands: Commands,
    query: Query<&Calc>,
    mut query_out: Query<&mut OutCheck>,
    mut query_datas: Query<&mut Datas>,
    query_copy: Query<&Copy>,
    query_print: Query<&Print>,
) {
    query.for_each_mut(|mut calc| match calc.input_type {
        Some(t) => match t {
            a if a == TypeId::of::<i32>() => calc_job!(calc, query_out, query_datas, [i32]),
            a if a == TypeId::of::<f32>() => calc_job!(calc, query_out, query_datas, [f32]),
            _ => (),
        },
        None => (),
    });
    // query_copy.for_each_mut(|copy| {
    //     {
    //         // println!("calc : {:?}", $calc.operation);
    //         let mut now_outcheck = query_out
    //             .get_mut(copy.output_outcheck_entity.unwrap())
    //             .unwrap();

    //         //if this node is already done
    //         if now_outcheck.node_done {
    //             // println!("this node is already done");
    //             // return;
    //         }
    //     }
    //     let mut input_1;
    //     check_and_get_value!(
    //         input_1,
    //         copy.input_a_entity,
    //         copy.input_a_outcheck_entity,
    //         mut query_out,
    //         query_datas,
    //         f32
    //     );

    //     let values_to_hand_over = input_1;

    //     let mut datas = query_datas.get_mut(copy.output_entity.unwrap()).unwrap();
    //     datas.f32 = Some(values_to_hand_over);

    //     let mut outcheck = query_out
    //         .get_mut(copy.output_outcheck_entity.unwrap())
    //         .unwrap();
    //     outcheck.node_done = true;
    // });
    // query_print.for_each_mut(|print| {
    //     let mut input_1;

    //     check_and_get_value!(
    //         input_1,
    //         print.input_a_entity,
    //         print.input_a_outcheck_entity,
    //         mut query_out,
    //         query_datas,
    //         f32
    //     );
    //     // println!("print: {}", input_1);
    // });
}
#[macro_export]
macro_rules! calc_job {
    ($calc:expr,$query_out:expr,$query_datas:expr, [$type_:ident]) => {{
        let mut input_1;
        let mut input_2;
        //TODO: 모듈화 ( if 문 생각하기)------------------>>
        {
            // println!("calc : {:?}", $calc.operation);
            let mut now_outcheck = $query_out
                .get_mut($calc.output_outcheck_entity.unwrap())
                .unwrap();

            //if this node is already done
            if now_outcheck.node_done {
                // println!("this node is already done");
                // return;
            }
        }
        //TODO: 모듈화 ------------------<<

        check_and_get_value!(
            input_1,
            $calc.input_a_entity,
            $calc.input_a_outcheck_entity,
            mut $query_out,
            $query_datas,
            $type_
        );

        //get data from datas entity
        //check whether outcheck exist
        check_and_get_value!(
            input_2,
            $calc.input_b_entity,
            $calc.input_b_outcheck_entity,
            mut $query_out,
            $query_datas,
            $type_
        );
        // println!("thers some value: {}, {}", input_1, input_2);

        let values_to_hand_over;
        values_to_hand_over = match $calc.operation {
            ArithmeticOperation::ADD => calc!(input_1 + input_2),
            ArithmeticOperation::SUBTRACT => calc!(input_1 - input_2),
            ArithmeticOperation::MULTIPLY => calc!(input_1 * input_2),
            ArithmeticOperation::DIVIDE => calc!(input_1 / input_2),
        };

        let mut datas = $query_datas.get_mut($calc.output_entity.unwrap()).unwrap();
        datas.$type_ = values_to_hand_over;

        let mut outcheck = $query_out
            .get_mut($calc.output_outcheck_entity.unwrap())
            .unwrap();
        outcheck.node_done = true;
    }};
}

#[macro_export]
macro_rules! calc {
    ($e:expr) => {{
        let val = $e;
        // println!("{} = {}", stringify!{$e}, val)
        Some(val)
    }};
}
fn print_system(
    mut commands: Commands,
    query: Query<&Print>,
    query_out: Query<&OutCheck>, // it doesn't need mutable. just for using macro
    mut query_datas: Query<&mut Datas>,
) {
    query.for_each_mut(|print| {
        let mut input_1;

        check_and_get_value!(
            input_1,
            print.input_a_entity,
            print.input_a_outcheck_entity,
            query_out,
            query_datas,
            f32
        );
        // println!("print: {}", input_1);
    });
}
fn copy_system(
    mut commands: Commands,
    query: Query<&Copy>,
    mut query_out: Query<&mut OutCheck>, // it doesn't need mutable. just for using macro
    mut query_datas: Query<&mut Datas>,
) {
    query.for_each_mut(|copy| {
        {
            // println!("calc : {:?}", $calc.operation);
            let mut now_outcheck = query_out
                .get_mut(copy.output_outcheck_entity.unwrap())
                .unwrap();

            //if this node is already done
            if now_outcheck.node_done {
                // println!("this node is already done");
                // return;
            }
        }
        let mut input_1;
        check_and_get_value!(
            input_1,
            copy.input_a_entity,
            copy.input_a_outcheck_entity,
            mut query_out,
            query_datas,
            f32
        );

        let values_to_hand_over = input_1;

        let mut datas = query_datas.get_mut(copy.output_entity.unwrap()).unwrap();
        datas.f32 = Some(values_to_hand_over);

        let mut outcheck = query_out
            .get_mut(copy.output_outcheck_entity.unwrap())
            .unwrap();
        outcheck.node_done = true;
    });
}

#[macro_export]
macro_rules! check_and_get_value {
    //mut
    ($container_for_value:ident,$value_entity:expr, $value_outcheck_entity:expr,mut $query_out:expr, $query_datas:expr, $type_:ident) => {
        //get data from datas entity
        //check whether outcheck exist
        if let Some(ex_out_check) = $query_out.get_mut($value_outcheck_entity.unwrap()).ok() {
            $container_for_value = match ex_out_check.node_done {
                true => $query_datas
                    .get_mut($value_entity.unwrap())
                    .unwrap()
                    .$type_
                    .unwrap(),
                _ => {
                    // error!("input_1 fail when get datas");
                    return;
                }
            };
        } else {
            // if outcheck doesn't exist then return
            // error!("input_1 fail when ex_outcheck done");
            return;
        }
    };
    // non mut
    ($container_for_value:ident, $value_entity:expr, $value_outcheck_entity:expr, $query_out:expr, $query_datas:expr, $type_:ident) => {
        //get data from datas entity
        //check whether outcheck exist
        if let Some(ex_out_check) = $query_out.get($value_outcheck_entity.unwrap()).ok() {
            $container_for_value = match ex_out_check.node_done {
                true => $query_datas
                    .get_mut($value_entity.unwrap())
                    .unwrap()
                    .$type_
                    .unwrap(),
                _ => {
                    // error!("input_1 fail when get datas");
                    return;
                }
            };
        } else {
            // if outcheck doesn't exist then return
            // error!("input_1 fail when ex_outcheck done");
            return;
        }
    };
}

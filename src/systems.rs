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
struct Connector {
    ex_nodes_id_done: HashMap<Uuid, bool>,
    now_node_id: Uuid,
    next_node_types_ids: HashMap<TypeId, Vec<Uuid>>,
    input_type: Option<TypeId>,
    input1_entity: Option<Entity>,
    input2_entity: Option<Entity>,
    output_value_entity_type: Option<(Entity, TypeId)>,
}

pub struct Connectors {
    hashmap: HashMap<TypeId, Vec<Connector>>,
}
#[derive(Debug)]
pub struct Calc {
    operation: ArithmeticOperation,
}
#[derive(Default)]
struct Data<T> {
    values: HashMap<Uuid, Option<T>>,
}

#[derive(Default)]
pub struct Datas {
    bool: Option<bool>,
    i32: Option<i32>,
    f32: Option<f32>,
    vector3: Option<Vec3>,
    string: Option<String>,
}

struct Print;

#[derive(Debug)]
enum ArithmeticOperation {
    ADD,
    SUBTRACT,
    MULTIPLY,
    DIVIDE,
}

pub fn setup_lerp(mut commands: Commands) {
    for i in 0..10000 {
        //---------------------------------------
        //Create Node UUID **node == job
        //---------------------------------------
        //make node
        let id_node_add = Uuid::new_v4();
        let id_node_subtract = Uuid::new_v4();
        let id_node_multiply = Uuid::new_v4();
        let id_node_print = Uuid::new_v4();

        //make data bank slot
        let id_value_a = Uuid::new_v4();
        let id_value_b = Uuid::new_v4();
        let id_value_t = Uuid::new_v4();
        let id_value_subtract_out = Uuid::new_v4();
        let id_value_add_out = Uuid::new_v4();
        let id_value_multiply_out = Uuid::new_v4();

        //---------------------------------------
        //Create Connector
        //---------------------------------------
        /*

         a      a      t
          \      \      \
        b -- sub -- add -- mul

        */
        //make node connect
        let mut connector_add = Connector {
            ex_nodes_id_done: [(id_node_subtract, false)].iter().cloned().collect(),
            now_node_id: id_node_add,
            next_node_types_ids: {
                let mut hash = HashMap::default();
                hash.insert(TypeId::of::<Calc>(), vec![id_node_multiply]);
                hash
            },
            input_type: None,
            input1_entity: None,
            input2_entity: None,
            output_value_entity_type: None,
        };

        let mut connector_subtract = Connector {
            ex_nodes_id_done: {
                let mut hash: HashMap<Uuid, bool> = HashMap::default();
                hash.insert(id_value_a, true);
                hash.insert(id_value_b, true);
                hash
            },
            now_node_id: id_node_subtract,
            next_node_types_ids: [(TypeId::of::<Calc>(), vec![id_node_add])]
                .iter()
                .cloned()
                .collect(),
            input_type: None,
            input1_entity: None,
            input2_entity: None,
            output_value_entity_type: None,
        };

        let mut connector_multiply = Connector {
            ex_nodes_id_done: [(id_node_add, false)].iter().cloned().collect(),
            now_node_id: id_node_multiply,
            next_node_types_ids: [(TypeId::of::<Calc>(), vec![id_node_print])]
                .iter()
                .cloned()
                .collect(),
            input_type: None,
            input1_entity: None,
            input2_entity: None,
            output_value_entity_type: None,
        };

        let mut connector_print = Connector {
            ex_nodes_id_done: [(id_node_multiply, false)].iter().cloned().collect(),
            now_node_id: id_node_multiply,
            next_node_types_ids: HashMap::default(),
            input_type: None,
            input1_entity: None,
            input2_entity: None,
            output_value_entity_type: None,
        };
        //---------------------------------------
        //Create Data<T> Hashmap
        //---------------------------------------
        /*  let mut data_value_a = Data::<f32> {
            values: [(id_value_a, Some(100.))].iter().cloned().collect(),
        };

        let mut data_value_b = Data::<f32> {
            values: [(id_value_b, Some(0.))].iter().cloned().collect(),
        };

        let mut data_value_t = Data::<f32> {
            values: [(id_value_t, Some(0.4))].iter().cloned().collect(),
        };
        let mut data_value_subtract_out = Data::<f32> {
            values: [(id_value_subtract_out, None)].iter().cloned().collect(),
        };
        let mut data_value_add_out = Data::<f32> {
            values: [(id_value_add_out, None)].iter().cloned().collect(),
        };
        let mut data_value_multiply_out = Data::<f32> {
            values: [(id_value_multiply_out, None)].iter().cloned().collect(),
        }; */

        let mut data_value_a = Some(100.);
        let mut data_value_b = Some(0.);
        let mut data_value_t = Some(0.4);
        let mut data_value_subtract_out = None;
        let mut data_value_add_out = None;
        let mut data_value_multiply_out = None;

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

        let entity_value_b = commands
            .spawn()
            .insert(Datas {
                f32: data_value_b,
                ..Default::default()
            })
            .id();

        let entity_value_t = commands
            .spawn()
            .insert(Datas {
                f32: data_value_t,
                ..Default::default()
            })
            .id();

        let entity_value_add_out = commands
            .spawn()
            .insert(Datas {
                f32: data_value_add_out,
                ..Default::default()
            })
            .id();

        let entity_value_subtract_out = commands
            .spawn()
            .insert(Datas {
                f32: data_value_subtract_out,
                ..Default::default()
            })
            .id();

        let entity_value_multiply_out = commands
            .spawn()
            .insert(Datas {
                f32: data_value_multiply_out,
                ..Default::default()
            })
            .id();

        //---------------------------------------
        //Add input, output data ids to connector
        //---------------------------------------
        //Inputs_type
        connector_add.input_type = Some(TypeId::of::<f32>());
        connector_add.input_type = Some(TypeId::of::<f32>());
        connector_subtract.input_type = Some(TypeId::of::<f32>());
        connector_subtract.input_type = Some(TypeId::of::<f32>());
        connector_multiply.input_type = Some(TypeId::of::<f32>());
        connector_multiply.input_type = Some(TypeId::of::<f32>());
        connector_print.input_type = Some(TypeId::of::<f32>());
        //Inputs_entity
        connector_add.input1_entity = Some(entity_value_a);
        connector_add.input2_entity = Some(entity_value_subtract_out);
        connector_subtract.input1_entity = Some(entity_value_b);
        connector_subtract.input2_entity = Some(entity_value_a);
        connector_multiply.input1_entity = Some(entity_value_add_out);
        connector_multiply.input1_entity = Some(entity_value_t);
        connector_print.input1_entity = Some(entity_value_multiply_out);

        //Ouputs
        connector_add.output_value_entity_type = Some((entity_value_add_out, TypeId::of::<f32>()));
        connector_subtract.output_value_entity_type =
            Some((entity_value_subtract_out, TypeId::of::<f32>()));
        connector_multiply.output_value_entity_type =
            Some((entity_value_multiply_out, TypeId::of::<f32>()));

        //---------------------------------------
        //Create Connectors hash and insert connector to each type
        //---------------------------------------

        let mut hash_add = HashMap::default();
        hash_add.insert(TypeId::of::<Calc>(), vec![connector_add]);

        let mut hash_sub = HashMap::default();
        hash_sub.insert(TypeId::of::<Calc>(), vec![connector_subtract]);

        let mut hash_mul = HashMap::default();
        hash_mul.insert(TypeId::of::<Calc>(), vec![connector_multiply]);

        //---------------------------------------
        //Create Node( job )-Connectors Entity
        //---------------------------------------
        // think that calc{ Add } and Calc{ Sub } is diffrent tag. This concept is temporarly
        commands
            .spawn()
            .insert(Connectors { hashmap: hash_add })
            .insert(Calc {
                operation: ArithmeticOperation::ADD,
            });
        commands
            .spawn()
            .insert(Connectors { hashmap: hash_sub })
            .insert(Calc {
                operation: ArithmeticOperation::SUBTRACT,
            });
        commands
            .spawn()
            .insert(Connectors { hashmap: hash_mul })
            .insert(Calc {
                operation: ArithmeticOperation::MULTIPLY,
            });
    }
}

use crate::{calc, node_job_basic, system_basic};

pub fn calc_system(
    mut commands: Commands,
    mut query: Query<(&Calc, &mut Connectors)>,
    mut query_datas: Query<&mut Datas>,
) {
    query.for_each_mut(|(node_tag, mut connectors)| {
        system_basic!(node_tag, connectors, query_datas);
    });
}

#[macro_export]
macro_rules! system_basic {
    ($tag:expr, $connectors:expr, $query_datas:expr) => {{
        let mut passed_node_ids_nextnodetypes: HashMap<Uuid, Vec<TypeId>> = HashMap::default();
        let mut list_connector = $connectors.hashmap.get(&$tag.type_id()).unwrap();
        for connector in list_connector {
            //---------------------------------------
            //Check if node is ready to process
            //---------------------------------------
            let true_nodes_count = connector
                .ex_nodes_id_done
                .values()
                .filter_map(|v| match v {
                    true => Some(true),
                    _ => None,
                })
                .collect::<Vec<_>>()
                .len();

            // skip if not match
            if connector.ex_nodes_id_done.len() != true_nodes_count {
                continue;
            }
            //-------------------
            //Node Job
            //-----------------

            //check whether it has value or not
            let input1_entity = match connector.input1_entity {
                Some(input_entity) => Some(input_entity),// if some, then return exactly.
                None => continue,// if none, then don't have to do anything. So, skip.
            };

            let input2_entity = connector.input2_entity; // if input2 is none, it has to some node thing but not with input2. so return exactly same.

            let outputs = connector.output_value_entity_type;

            match connector.input_type {
                Some(x) if x == TypeId::of::<i32>() => {
                    node_job_basic!(
                        $tag,
                        $query_datas,
                        [i32],
                        input1_entity,
                        input2_entity,
                        outputs
                    )
                }
                Some(x) if x == TypeId::of::<f32>() => {
                    node_job_basic!(
                        $tag,
                        $query_datas,
                        [f32],
                        input1_entity,
                        input2_entity,
                        outputs
                    )
                }
                _ => (),
            }
            //pass for next node
            passed_node_ids_nextnodetypes.insert(
                connector.now_node_id,
                connector.next_node_types_ids.keys().cloned().collect(),
            );
        }
        for passed_node_id in passed_node_ids_nextnodetypes.keys() {
            let next_types = passed_node_ids_nextnodetypes.get(passed_node_id).unwrap();
            for next_type in next_types {
                let mut list_next_connector = $connectors.hashmap.get_mut(next_type).unwrap();
                for mut next_connector in list_next_connector.iter_mut() {
                    if let Some(mut ex_node_of_next_node) =
                        next_connector.ex_nodes_id_done.get(passed_node_id)
                    {
                        //참조라 안된다. insert로 해쉬맵을 갱신해줘야함
                        next_connector
                            .ex_nodes_id_done
                            .insert(passed_node_id.clone(), true);
                        // println!("match: now_node & next's ex_node");
                        ex_node_of_next_node = &true;
                        // println!("ex_node_of_next_node new set :{}", ex_node_of_next_node);
                    } else {
                        continue;
                    }
                }
            }
        }
    }};
}

#[macro_export]
macro_rules! node_job_basic {
    ( $tag:expr, $query_datas:expr, [$type_:ident],
    $input1_entity:expr,
    $input2_entity:expr,$output_data:expr) => {{
        let mut input_1;
        let mut input_2;

        //get data from datas entity
        //it is obious that input1_entity is not none
        match $query_datas.get_mut($input1_entity.unwrap()) {
            Ok(datas) => {
                input_1 = datas.$type_;
            }
            _ => {
                input_1 = None;
            }
        };

        //it is suspicious that input2_entity has value. so check and get
        if let Some(input2_entity) = $input2_entity {
            match $query_datas.get_mut(input2_entity) {
                Ok(datas) => {
                    input_2 = datas.$type_;
                }
                _ => {
                    input_2 = None;
                    println!("input1_entity is exist but value is not");
                }
            };
        } else {
            input_2 = None;
        }

        let input1_unwrap;
        let values_to_hand_over_1;
        input1_unwrap = input_1.expect("input1_entity is exist but value is not");

        //if value 2 is exist, then can do dual input function
        if let Some(input2_unwrap) = input_2 {
            values_to_hand_over_1 = match $tag.operation {
                ArithmeticOperation::ADD => calc!(input1_unwrap + input2_unwrap),
                ArithmeticOperation::SUBTRACT => calc!(input1_unwrap - input2_unwrap),
                ArithmeticOperation::MULTIPLY => calc!(input1_unwrap * input2_unwrap),
                ArithmeticOperation::DIVIDE => calc!(input1_unwrap / input2_unwrap),
            };
        }
        //if value 2 is none, then can do solo input function
        else {
            values_to_hand_over_1 = None;
        }

        match $output_data {
            Some((entity, typeid)) => {
                let mut datas = $query_datas.get_mut(entity).unwrap();
                datas.$type_ = values_to_hand_over_1;
            }
            None => (),
        }
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

use std::collections::HashMap;

use rand;

#[derive(Debug)]
struct Entity {
    id: u32,
}

#[derive(Debug)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Velocity {
    x: i32,
    y: i32,
}

fn main() {
    
    // 現時点で作られたエンティティの数を表す。
    // それと、次に作るエンティティのidも表す。
    let mut entity_count = 0;

    // ベクタをやめて、ハッシュマップで宣言する
    // Position構造体のハッシュマップ
    let mut pos_ary: HashMap<u32, Position> = HashMap::new();  

    // Velocity構造体のハッシュマップ
    let mut vel_ary: HashMap<u32, Velocity> = HashMap::new();
    
    // 効果を確かめるために、2回づつ create_entity を呼び出す
    // Positionコンポーネントのみ
    for _ in 0..2 {
        let (x, y) = rand::random();
        let entity = create_entity(&mut entity_count);
        add_component(&entity, &mut pos_ary, Position { x, y });
    }

    // Velocityコンポーネントのみ
    for _ in 0..2 {
        let (x, y) = rand::random();
        let entity = create_entity(&mut entity_count);
        add_component(&entity, &mut vel_ary, Velocity{ x, y });
    }

    // PositionとVelocityの両方
    for _ in 0..2 {
        let (x ,y) = rand::random();
        let entity = create_entity(&mut entity_count);
        add_component(&entity, &mut pos_ary, Position { x, y });
        add_component(&entity, &mut vel_ary, Velocity { x, y })
    }


    // 各要素の値を出力する
    for i in 0..entity_count  {
        let pos = pos_ary.get(&i);
        let vel = vel_ary.get(&i);
        
        println!("{}: {:?}", i, pos);
        println!("{}: {:?}", i, vel);
    }
}

// 現在のentity_countをidにしたエンティティを返す
fn create_entity (id: &mut u32) -> Entity {
    let entity = Entity { id: *id };
    *id += 1;
    entity
}

// ハッシュマップにコンポーネントを追加する
fn add_component <T> (entity: &Entity, v: &mut HashMap<u32, T>, data: T) {
    v.insert(entity.id, data);
}

fn remove_component <T> (entity: &Entity, compornents: &mut HashMap<u32, T>) {
    compornents.remove(&entity.id);
}

fn destroy_entity <T, U> (entity: &Entity, compornents1: &mut HashMap<u32, T>, compornents2: &mut HashMap<u32, U>) {
    compornents1.remove(&entity.id);
    compornents2.remove(&entity.id);
}

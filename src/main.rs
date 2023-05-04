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
    for i in 0..2 {
        let (x, y) = rand::random();
        let key = create_entity(&mut entity_count);
        add_component(&key, &mut pos_ary, Position { x, y });
    }

    // Velocityコンポーネントのみ
    for i in 0..2 {
        let (x, y) = rand::random();
        let key = create_entity(&mut entity_count);
        add_component(&key, &mut vel_ary, Velocity{ x, y });
    }

    // PositionとVelocityの両方
    for i in 0..2 {
        let (x ,y) = rand::random();
        let key = create_entity(&mut entity_count);
        add_component(&key, &mut pos_ary, Position { x, y });
        add_component(&key, &mut vel_ary, Velocity { x, y })
    }


    // 各要素の値を出力する
    for i in 0..entity_count  {
        let pos = pos_ary.get(&i);
        let vel = vel_ary.get(&i);
        
        println!("{}: {:?}", i, pos);
        println!("{}: {:?}", i, vel);
    }
}

// id に +1 して作成したエンティティを返す
fn create_entity (id: &mut u32) -> u32 {
    let key = *id;
    *id += 1;
    key
}

// ハッシュマップにコンポーネントを追加する
fn add_component <T> (id: &u32, v: &mut HashMap<u32, T>, data: T) {
    v.insert(*id, data);
}

// ジェネリックなデータ型を引数に取る set_data 関数
fn generic_set_data <T> (id: &usize, v: &mut Vec<T>, data: T) {
    v[*id] = data;
}

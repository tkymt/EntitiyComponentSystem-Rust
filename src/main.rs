use rand;

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
    
    // idは配列の要素数を表す
    let mut id = 0;

    // 配列をやめて、ベクタで宣言する
    // Position構造体のベクタ
    let mut pos_ary: Vec<Position> = vec![];  

    // Velocity構造体のベクタ
    let mut vel_ary: Vec<Velocity> = vec![];
    
    // 効果を確かめるために、10回 create_entity を呼び出す
    for i in 0..10 {
        // ランダムな値を取得する
        let (x, y) = rand::random();
        create_entity(&mut id, &mut pos_ary,);
        let t = id -1;
        set_data(&t, &mut pos_ary,Position{ x, y });
    }



    for i in 0..id  {
        println!("{}: {:?}", i, pos_ary[i])
    }
}

// idを増やして、配列に要素を追加する
fn create_entity(id: &mut usize, ary: &mut Vec<Position>){
    *id += 1;
    ary.push(Position { x: 0, y: 0 });
}

// idを増やして、配列に指定された値の要素を追加する
fn create_entity_pos(id: &mut usize, ary: &mut Vec<Position>, data: Position) {
    *id += 1;
    ary.push(data);
}

// 配列の指定されたidの要素に、指定された値を代入する
fn set_data (id: &usize, ary: &mut Vec<Position>, data: Position) {
    ary[*id] = data;
}

fn create_entity_vel_zero_init(id: &mut usize, ary: &mut Vec<Velocity>) {
    *id += 1;
    ary.push(Velocity { x: 0, y: 0 });
}

fn create_entity_vel(id: &mut usize, ary: &mut Vec<Velocity>, data: Velocity) {
    *id += 1;
    ary.push(data);
}

fn set_data_vel(id: &usize, ary: &mut Vec<Velocity>, data: Velocity) {
    ary[*id] = data;
}
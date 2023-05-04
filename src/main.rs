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
        generic_create_entity(&mut id, &mut pos_ary, Position { x, y });
        generic_create_entity(&mut id, &mut vel_ary, Velocity { x, y });
     }


    // ベクタの要素を出力する
    for i in 0..id  {
        println!("{}: {:?}", i, pos_ary[i]);
        println!("{}: {:?}", i, vel_ary[i]);
    }
}

// ベクタにコンポーネントをを追加するのをやめて、id に +1 するだけにした
fn create_entity <T> (id: &mut usize) {
    *id += 1;
}

// ベクタにコンポーネントを追加する
fn add_component <T> (v: &mut Vec<T>, data: T) {
    v.push(data)
}

// ジェネリックなデータ型を引数に取る set_data 関数
fn generic_set_data <T> (id: &usize, v: &mut Vec<T>, data: T) {
    v[*id] = data;
}

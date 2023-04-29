fn main() {
    
    // idは配列の要素数を表す
    let mut id = 0;

    let mut pos_ary: Vec<(i32, i32)> = vec![];

    for i in 0..10 {
        create_entity(&mut id, &mut pos_ary);
    }

    for i in 0..id  {
        println!("{}: {:?}", i, pos_ary[i])
    }
}

// idを増やして、配列に要素を追加する
fn create_entity(id: &mut usize, ary: &mut Vec<(i32, i32)>){
    *id += 1;
    ary.push((0, 0));
}

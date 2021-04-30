struct Pair(i32, f32);

pub fn print_destruct(){
    let pair = Pair(1, 0.1);
    println!("pair contains {:?} and {:?}", pair.0, pair.1);
    let Pair(integer, decimal) = pair;
    println!("pair contains {:?} and {:?}", integer, decimal);
    println!("pair contains {:?} and {:?}", pair.0, pair.1);
}
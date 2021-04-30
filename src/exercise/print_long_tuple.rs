#[derive(Debug)]
struct LngTuple(&'static str, u8, i32, i32, i32, i32, i32, i32,
                i32, i32, i32, i32, i32, i32, i32, i32, i32, bool);



pub fn print_tuple() {
    let too_long_tuple = LngTuple("hell", 2, 3, 4, 5, 6, 7, 8,
                                  9, 10, 11, 12, 13, 14, 15, 16, 17, true);
    println!("{:#?}", too_long_tuple);
}
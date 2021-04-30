mod exercise;

use crate::exercise::rough_pi::print_rough_pi;
use crate::exercise::print_vector::print_vector;
use crate::exercise::print_long_tuple::print_tuple;
use crate::exercise::destruct_struct::print_destruct;
use crate::exercise::enum_print::print_enum;

fn main() {
    print_rough_pi();
    print_vector();
    print_tuple();
    print_destruct();
    print_enum();
}

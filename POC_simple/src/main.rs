mod simple_1;
mod simple_2;

fn main() {
    let flag_simple1: bool = true;
    let flag_simple2: bool = false;

    simple_1::run_simple1(flag_simple1);
    println!("\n\nSimple 2: \n");
    simple_2::run_simple2(flag_simple2);
}

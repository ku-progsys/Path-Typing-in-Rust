use rand::Rng;

use secret_structs::lattice as lat3;
use secret_structs::secret as st;
use secret_macros::secret_block;


fn main() {
    // define secret value 
    let mut sec_val = secret_block!(lat3::Label_AB { wrap_secret(0) });
    let mut rng = rand::thread_rng();
    let rand_num: i32 = rng.gen_range(0..101);
    println!("random num: {}", rand_num);
    if rand_num  % 2 == 0 {
        secret_block!(lat3::Label_AB {
            *unwrap_secret_mut_ref(&mut sec_val) += 1;
        });
    }
    else {
        // declassify 
        println!("Non-secret value: {:?}", sec_val.declassify());
    }
}

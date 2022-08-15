use rand::Rng;

use primitives;
//use utils;


fn main() {

    for _i in 1..20 {
        //println!("Hello world");
        let rnd_tmp = rand::thread_rng().gen_range(0..(1 << 100 - 1));
        //let fn_result = primitives::one_way_fn(rnd_tmp) >> 64;
        //let hcp_result = primitives::hcp(rnd_tmp);
        //println!("f({0}) = {1}, hcp(f({0})) = {2}", rnd_tmp, fn_result, hcp_result);

        let (g1, g2) = primitives::prg(rnd_tmp);
        println!("x = {0}, G(x) = {1}, {2}", rnd_tmp, g1, g2);
        //println!("x = {0:#b}, G(x) = {1:#b}{2:#b}", rnd_tmp, g1, g2);
    }


    
}

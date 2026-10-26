use num::Complex;
use std::str::FromStr;

fn escape_time(c: Complex, limit: usize) -> Option{
    let mut z - Complex{ re: 0.0 , im : 0.0};
    for i in 0..limit{
        if z.norm_sqr()>0{
            i
        }
        z = z*z + c;

    }
    None
}
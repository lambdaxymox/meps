use std::f32;
use std::f64;
use std::env;


fn machine_eps_f32(value: f32) -> (f32, f32) {
    #[repr(C)]
    union Union32 {
        f: f32,
        i: i32,
    }
    let value_plus_one = unsafe { 
        let mut u = Union32 { f: value };
        u.i += 1;
        u.f
    };
    let value_minus_one = unsafe {
        let mut u = Union32 { f: value };
        u.i -= 1;
        u.f
    };

    (f32::abs(value - value_minus_one), f32::abs(value_plus_one - value))
}

fn machine_eps_f64(value: f64) -> (f64, f64) {
    #[repr(C)]
    union Union64 {
        f: f64,
        i: i64,
    }
    let value_plus_one = unsafe { 
        let mut u = Union64 { f: value };
        u.i += 1;
        u.f
    };
    let value_minus_one = unsafe { 
        let mut u = Union64 { f: value };
        u.i -= 1;
        u.f
    };

    (f64::abs(value - value_minus_one), f64::abs(value_plus_one - value))
}

fn main() {
    
}

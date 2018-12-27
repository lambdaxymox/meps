use std::f32;
use std::f64;
use std::env;
use std::fmt;
use std::process;


#[derive(Copy, Clone, Debug, PartialEq)]
struct Bounded32 {
    value: f32,
    lower: f32,
    upper: f32,
}

impl fmt::Display for Bounded32 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} (+{}) (-{})", self.value, self.upper, self.lower)
    }
}

fn machine_eps_f32(value: f32) -> Bounded32 {
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

    Bounded32 {
        value: value,
        lower: f32::abs(value - value_minus_one),
        upper: f32::abs(value_plus_one - value)
    }
}


#[derive(Copy, Clone, Debug, PartialEq)]
struct Bounded64 {
    value: f64,
    lower: f64,
    upper: f64,
}

impl fmt::Display for Bounded64 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} (+{}) (-{})", self.value, self.upper, self.lower)
    }
}

fn machine_eps_f64(value: f64) -> Bounded64 {
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

    Bounded64 {
        value: value,
        lower: f64::abs(value - value_minus_one),
        upper: f64::abs(value_plus_one - value),
    }
}

fn usage() -> String {
    format!("USAGE: meps <floating point number>")
}

fn main() {
    let args = env::args().collect::<Vec<String>>();
    if args.len() != 2 {
        println!("{}", usage());
        process::exit(1);
    }
    
    let num_str =  &args[1];
    let maybe_num_f32 = str::parse::<f32>(&num_str); 
    let maybe_num_f64 = str::parse::<f64>(&num_str);

    if maybe_num_f32.is_err() || maybe_num_f64.is_err() {
        eprintln!("The value {} does not represent a valid floating point number.", num_str);
        process::exit(1);
    }

    let num_f32 = maybe_num_f32.unwrap();
    let num_f64 = maybe_num_f64.unwrap();

    println!("{} at 32 bits", machine_eps_f32(num_f32));
    println!("{} at 64 bits", machine_eps_f64(num_f64));
}

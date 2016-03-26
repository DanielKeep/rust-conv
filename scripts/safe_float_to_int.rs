// cargo-deps: ieee754="0.2.1"
/*!
Experimentally derives, for each (int, float) pair, the largest and smallest integer values that survive round-tripping through the float types.

This is to verify *exactly* what the safe range for float-to-int conversions is.
*/
extern crate ieee754;
use std::fmt;
use ieee754::Ieee754;

macro_rules! limits {
    ($src:ty => $dst:ident; < $min:expr, > $max:expr) => {
        limits!($src => $dst; < $min);
        limits!($src => $dst; > $max);
    };
    ($src:ty => $dst:ident; > $max:expr) => {
        {
            let mut cur: $src = $max;
            if ((cur as $dst) as $src) != cur {
                panic!("safe {} max: not found; initial limit too high!", stringify!($src => $dst));
            }
            loop {
                let next = (cur + 1.0).max(cur.next());
                let next_int = next as $dst;
                let next_rnd = next_int as $src;
                if next_rnd != next {
                    println!("safe {} max: {}, {:e}, {:+x}",
                        stringify!($src => $dst), cur, cur, FloatHex(cur));
                    break;
                } else {
                    cur = next;
                }
            }
        }
    };
    ($src:ty => $dst:ident; < $min:expr) => {
        {
            let mut cur: $src = $min;
            if ((cur as $dst) as $src) != cur {
                panic!("safe {} min: not found; initial limit too low!", stringify!($src => $dst));
            }
            loop {
                let next = (cur - 1.0).min(cur.prev());
                let next_int = next as $dst;
                let next_rnd = next_int as $src;
                if next_rnd != next {
                    println!("\rsafe {} min: {:+}, {:+e}, {:+x}",
                        stringify!($src => $dst), cur, cur, FloatHex(cur));
                    break;
                } else {
                    cur = next;
                }
            }
        }
    };
}

fn main() {
    limits!(f32 => i8; < -120.0, > 120.0);
    limits!(f32 => i16; < -32000.0, > 32000.0);
    limits!(f32 => i32; < -2147480000.0, > 2147480000.0);
    limits!(f32 => i64; < -9223300000000000000.0, > 9223300000000000000.0);
    limits!(f32 => u8; > 250.0);
    limits!(f32 => u16; > 64000.0);
    limits!(f32 => u32; > 4290000000.0);
    limits!(f32 => u64; > 18446700000000000000.0);

    limits!(f64 => i8; < -120.0, > 120.0);
    limits!(f64 => i16; < -32000.0, > 32000.0);
    limits!(f64 => i32; < -2147480000.0, > 2147480000.0);
    limits!(f64 => i64; < -9223372036854770000.0, > 9223372036854700000.0);
    limits!(f64 => u8; > 250.0);
    limits!(f64 => u16; > 64000.0);
    limits!(f64 => u32; > 4290000000.0);
    limits!(f64 => u64; > 18446744073709500000.0);
}

struct FloatHex<T>(pub T);

impl fmt::LowerHex for FloatHex<f32> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        use std::num::FpCategory;

        fn write_sig(fmt: &mut fmt::Formatter, sig: u32) -> fmt::Result {
            let mut sig = sig << 9;
            loop {
                let nib = sig >> 28;
                try!(fmt.write_str(match nib {
                    0 => "0", 1 => "1", 2 => "2", 3 => "3",
                    4 => "4", 5 => "5", 6 => "6", 7 => "7",
                    8 => "8", 9 => "9", 10 => "a", 11 => "b",
                    12 => "c", 13 => "d", 14 => "e", _ => "f",
                }));
                sig <<= 4;
                if sig == 0 { break; }
            }
            Ok(())
        }

        fn write_exp(fmt: &mut fmt::Formatter, exp: i16) -> fmt::Result {
            try!(write!(fmt, "p{}", exp));
            Ok(())
        }

        let v = self.0;

        match v.classify() {
            FpCategory::Nan => {
                try!(fmt.write_str("nan"));
            },
            FpCategory::Infinite => {
                if v.is_sign_negative() {
                    try!(fmt.write_str("-"));
                } else if fmt.sign_plus() {
                    try!(fmt.write_str("+"));
                }
                try!(fmt.write_str("infinity"));
            },
            FpCategory::Zero => {
                if v.is_sign_negative() {
                    try!(fmt.write_str("-"));
                } else if fmt.sign_plus() {
                    try!(fmt.write_str("+"));
                }
                try!(fmt.write_str("0x0p0"));
            },
            FpCategory::Subnormal => {
                let (neg, exp, sig) = v.decompose();
                if neg { try!(fmt.write_str("-")); }
                else if fmt.sign_plus() { try!(fmt.write_str("+")); }
                try!(fmt.write_str("0x0."));
                try!(write_sig(fmt, sig));
                try!(write_exp(fmt, exp));
            },
            FpCategory::Normal => {
                let (neg, exp, sig) = v.decompose();
                if neg { try!(fmt.write_str("-")); }
                else if fmt.sign_plus() { try!(fmt.write_str("+")); }
                try!(fmt.write_str("0x1."));
                try!(write_sig(fmt, sig));
                try!(write_exp(fmt, exp));
            },
        }

        Ok(())
    }
}

impl fmt::LowerHex for FloatHex<f64> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        use std::num::FpCategory;

        fn write_sig(fmt: &mut fmt::Formatter, sig: u64) -> fmt::Result {
            let mut sig = sig << 13;
            loop {
                let nib = sig >> 60;
                try!(fmt.write_str(match nib {
                    0 => "0", 1 => "1", 2 => "2", 3 => "3",
                    4 => "4", 5 => "5", 6 => "6", 7 => "7",
                    8 => "8", 9 => "9", 10 => "a", 11 => "b",
                    12 => "c", 13 => "d", 14 => "e", _ => "f",
                }));
                sig <<= 4;
                if sig == 0 { break; }
            }
            Ok(())
        }

        fn write_exp(fmt: &mut fmt::Formatter, exp: i16) -> fmt::Result {
            try!(write!(fmt, "p{}", exp));
            Ok(())
        }

        let v = self.0;

        match v.classify() {
            FpCategory::Nan => {
                try!(fmt.write_str("nan"));
            },
            FpCategory::Infinite => {
                if v.is_sign_negative() {
                    try!(fmt.write_str("-"));
                } else if fmt.sign_plus() {
                    try!(fmt.write_str("+"));
                }
                try!(fmt.write_str("infinity"));
            },
            FpCategory::Zero => {
                if v.is_sign_negative() {
                    try!(fmt.write_str("-"));
                } else if fmt.sign_plus() {
                    try!(fmt.write_str("+"));
                }
                try!(fmt.write_str("0x0p0"));
            },
            FpCategory::Subnormal => {
                let (neg, exp, sig) = v.decompose();
                if neg { try!(fmt.write_str("-")); }
                else if fmt.sign_plus() { try!(fmt.write_str("+")); }
                try!(fmt.write_str("0x0."));
                try!(write_sig(fmt, sig));
                try!(write_exp(fmt, exp));
            },
            FpCategory::Normal => {
                let (neg, exp, sig) = v.decompose();
                if neg { try!(fmt.write_str("-")); }
                else if fmt.sign_plus() { try!(fmt.write_str("+")); }
                try!(fmt.write_str("0x1."));
                try!(write_sig(fmt, sig));
                try!(write_exp(fmt, exp));
            },
        }

        Ok(())
    }
}

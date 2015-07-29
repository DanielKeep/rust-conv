extern crate conv;

#[macro_use] mod util;

use conv::*;

use conv::FloatError::Underflow as FU;
use conv::FloatError::Overflow as FO;

#[test]
fn test_f32() {
    check!(f32, f32; fident; qv: *;);
    check!(f32, f64; fident; qv: *;);
}

#[test]
fn test_f32_to_int() {
    check!(f32, i8;  sidenta; qa: i8;  a: -129.0, !FU; a: 128.0, !FO;);
    check!(f32, i16; sidenta; qa: i16; a: -32_769.0, !FU; a: 32_768.0, !FO;);
    check!(f32, i32; sidenta; qa: i32; a: -2_147_500_000.0, !FU; a: 2_147_500_000.0, !FO;);
    check!(f32, i64; sidenta; qa: i64; a: -9_223_373_000_000_000_000.0, !FU; a: 9_223_373_000_000_000_000.0, !FO;);
    check!(f32, u8;  uidenta; qa: u8;  a: -1.0, !FU; a: 256.0, !FO;);
    check!(f32, u16; uidenta; qa: u16; a: -1.0, !FU; a: 65_536.0, !FO;);
    check!(f32, u32; uidenta; qa: u32; a: -1.0, !FU; a: 4_294_968_000.0, !FO;);
    check!(f32, u64; uidenta; qa: u64; a: -1.0, !FU; a: 18_446_746_000_000_000_000.0, !FO;);
}

#[test]
fn test_f64_to_int() {
    check!(f64, i8;  sidenta; qa: i8;  a: -129.0, !FU; a: 128.0, !FO;);
    check!(f64, i16; sidenta; qa: i16; a: -32_769.0, !FU; a: 32_768.0, !FO;);
    check!(f64, i32; sidenta; qa: i32; a: -2_147_483_649.0, !FU; a: 2_147_483_648.0, !FO;);
    check!(f64, i64; sidenta; qa: i64; a: -9_223_372_036_854_778_000.0, !FU; a: 9_223_372_036_854_778_000.0, !FO;);
    check!(f64, u8;  uidenta; qa: u8;  a: -1.0, !FU; a: 256.0, !FO;);
    check!(f64, u16; uidenta; qa: u16; a: -1.0, !FU; a: 65_536.0, !FO;);
    check!(f64, u32; uidenta; qa: u32; a: -1.0, !FU; a: 4_294_967_296.0, !FO;);
    check!(f64, u64; uidenta; qa: u64; a: -1.0, !FU; a: 18_446_744_073_709_560_000.0, !FO;);
}

#[test]
fn test_f64() {
    check!(f64, f32; fidenta; qa: *;);
    check!(f64, f64; fident; qv: *;);
}

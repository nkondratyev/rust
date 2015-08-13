// Copyright 2015 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![feature(repr_simd, platform_intrinsics, concat_idents,
           type_macros, test)]
#![allow(non_camel_case_types)]

extern crate test;

#[repr(simd)]
#[derive(PartialEq)]
struct i32x4(i32, i32, i32, i32);
#[repr(simd)]
#[derive(PartialEq)]
struct i8x4(i8, i8, i8, i8);

#[repr(simd)]
#[derive(PartialEq)]
struct u32x4(u32, u32, u32, u32);
#[repr(simd)]
#[derive(PartialEq)]
struct u8x4(u8, u8, u8, u8);

#[repr(simd)]
#[derive(PartialEq)]
struct f32x4(f32, f32, f32, f32);

#[repr(simd)]
#[derive(PartialEq)]
struct f64x4(f64, f64, f64, f64);


extern "platform-intrinsic" {
    fn simd_cast<T, U>(x: T) -> U;
}

const A: i32 = -1234567;
const B: i32 = 12345678;
const C: i32 = -123456789;
const D: i32 = 1234567890;

fn main() {
    macro_rules! test {
        ($from: ident, $to: ident) => {{
            // force the casts to actually happen, or else LLVM/rustc
            // may fold them and get slightly different results.
            let (a, b, c, d) = test::black_box((A as $from, B as $from, C as $from, D as $from));
            // the SIMD vectors are all FOOx4, so we can concat_idents
            // so we don't have to pass in the extra args to the macro
            let from = simd_cast(concat_idents!($from, x4)(a, b, c, d));
            let to = concat_idents!($to, x4)(a as $to,
                                             b as $to,
                                             c as $to,
                                             d as $to);
            assert!(to == from,
                    "{} -> {}", stringify!($from), stringify!($to));
        }}
    }
    macro_rules! tests {
        (: $($to: ident),*) => { () };
        // repeating the list twice is easier than writing a cartesian
        // product macro
        ($from: ident $(, $from_: ident)*: $($to: ident),*) => {
            fn $from() { unsafe { $( test!($from, $to); )* } }
            tests!($($from_),*: $($to),*)
        };
        ($($types: ident),*) => {{
            tests!($($types),* : $($types),*);
            $($types();)*
        }}
    }

    // test various combinations, including truncation,
    // signed/unsigned extension, and floating point casts.
    tests!(i32, i8, u32, u8, f32);
    tests!(i32, u32, f32, f64)
}

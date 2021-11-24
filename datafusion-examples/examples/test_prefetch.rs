#![feature(core_intrinsics)]
#![feature(link_llvm_intrinsics)]
use std::collections::{HashMap, HashSet};
use rand::Rng;
use tonic::server::StreamingService;
use chrono::prelude::*;

use core::arch::x86_64::_mm_prefetch;
use std::arch::x86_64::{_MM_HINT_NTA, _MM_HINT_T0, _MM_HINT_T2};
use std::ops::{Add, Index};
extern crate rand;

use std::intrinsics::prefetch_read_data;
use std::io::Read;
use std::mem;
// adapted from llvmint generated bindings to remove the dependency
extern {
    #[link_name = "llvm.prefetch"]
    fn llvm_prefetch(a: *mut i8, b: i32, c: i32, d: i32) -> ();
}

#[tokio::main]
async fn main() {
    let s: &str = "c23";
    let ptr = s.as_ptr() as *const i8;




    unsafe {
        //println!("{}", *ptr as char);
        println!("{}",((std::mem::size_of::<u64>() - 1) / 64 + 1));
        println!("{}", *ptr);
        //println!("{}", *ptr.offset(2) as char);
    }

    test1();
}
struct Dataline {
    u1: u64,
    u2: u64,
    u3: u64,
    u4: u64,
    u5: u64,
    u6: u64,
    u7: u64
}
fn test1() {
    let mut vec_data = Vec::new();
    let mut vec_index = Vec::new();

    let mut rng = rand::thread_rng();
    let mut i =0;
    while i < 100000000 {
        let rag_data = rng.gen::<u64>();
        let rag_index = rng.gen_range(0, 100000000);
        let dl = Dataline{
            u1:rag_data,
            u2:rag_data+1,
            u3:rag_data+2,
            u4:rag_data+3,
            u5:rag_data+4,
            u6:rag_data+5,
            u7:rag_data+6
        };
        vec_data.push(dl);
        vec_index.push(rag_index.clone());
        i+=1;
    }

    println!("build data successful");



    // let dt = Local::now();
    // let mut sum = 0;
    // let mut s = 0;
    // let len = vec_data.len();
    // vec_index.iter().enumerate().for_each(|(index,value)|{
    //     if(s < len) {
    //         sum = vec_data[vec_index[index]].u1
    //     }
    // });
    // println!("{}",sum);
    // println!(
    //     "usage millis: {}",
    //     Local::now().timestamp_millis() - dt.timestamp_millis()
    // );



    let dt = Local::now();
    let mut sum = 0;
    let mut s = 16;
    let len = vec_index.len();
    vec_index.iter().enumerate().for_each(|(index,value)|{
        sum = vec_data[vec_index[index]].u1;
        if(s < len){
            let x = unsafe {
                prefetch(&vec_data[vec_index[s]] as *const Dataline);
            };
            s+=1;
        }
    });
    println!("{}",sum);
    println!(
        "usage millis: {}",
        Local::now().timestamp_millis() - dt.timestamp_millis()
    );






}

fn prefetch(p: *const Dataline){
    let i8p = p as *const i8;
    unsafe {
        //_mm_prefetch(i8p.offset(-1), _MM_HINT_T0);
        _mm_prefetch(i8p, _MM_HINT_T0);
        //_mm_prefetch(i8p.offset(1), _MM_HINT_T0);
        // _mm_prefetch(i8p.offset(1), _MM_HINT_NTA);
        // _mm_prefetch(i8p.offset(2), _MM_HINT_NTA);
        // _mm_prefetch(i8p.offset(3), _MM_HINT_NTA);
        // _mm_prefetch(i8p.offset(4), _MM_HINT_NTA);
        // _mm_prefetch(i8p.offset(5), _MM_HINT_NTA);
        // _mm_prefetch(i8p.offset(6), _MM_HINT_NTA);
        // _mm_prefetch(i8p.offset(7), _MM_HINT_NTA);
    }
}


#[macro_export]
macro_rules! unroll {
    (0, |$i:ident| $s:stmt) => {};
    (1, |$i:ident| $s:stmt) => {{
        let $i: isize = 0;
        $s
    }};
    (2, |$i:ident| $s:stmt) => {{
        unroll!(1, |$i| $s);
        let $i: isize = 1;
        $s
    }};
    (3, |$i:ident| $s:stmt) => {{
        unroll!(2, |$i| $s);
        let $i: isize = 2;
        $s
    }};
    (4, |$i:ident| $s:stmt) => {{
        unroll!(3, |$i| $s);
        let $i: isize = 3;
        $s
    }};
    (5, |$i:ident| $s:stmt) => {{
        unroll!(4, |$i| $s);
        let $i: isize = 4;
        $s
    }};
    (6, |$i:ident| $s:stmt) => {{
        unroll!(5, |$i| $s);
        let $i: isize = 5;
        $s
    }};
    (7, |$i:ident| $s:stmt) => {{
        unroll!(6, |$i| $s);
        let $i: isize = 6;
        $s
    }};
    (8, |$i:ident| $s:stmt) => {{
        unroll!(7, |$i| $s);
        let $i: isize = 7;
        $s
    }};
    (9, |$i:ident| $s:stmt) => {{
        unroll!(8, |$i| $s);
        let $i: isize = 8;
        $s
    }};
    (10, |$i:ident| $s:stmt) => {{
        unroll!(9, |$i| $s);
        let $i: isize = 9;
        $s
    }};
    (11, |$i:ident| $s:stmt) => {{
        unroll!(10, |$i| $s);
        let $i: isize = 10;
        $s
    }};
    (12, |$i:ident| $s:stmt) => {{
        unroll!(11, |$i| $s);
        let $i: isize = 11;
        $s
    }};
    (13, |$i:ident| $s:stmt) => {{
        unroll!(12, |$i| $s);
        let $i: isize = 12;
        $s
    }};
    (14, |$i:ident| $s:stmt) => {{
        unroll!(13, |$i| $s);
        let $i: isize = 13;
        $s
    }};
    (15, |$i:ident| $s:stmt) => {{
        unroll!(14, |$i| $s);
        let $i: isize = 14;
        $s
    }};
    (16, |$i:ident| $s:stmt) => {{
        unroll!(15, |$i| $s);
        let $i: isize = 15;
        $s
    }};
}

const fn n_lines<T>() -> isize {
    ((std::mem::size_of::<T>() - 1) / 64 + 1) as isize
}

pub fn prefetchc<T>(p: *const T) {
    unsafe {
        match n_lines::<T>() {
            1 => unroll!(1, |i| core::arch::x86_64::_mm_prefetch(
                (p as *const i8).offset(i * 64),
                core::arch::x86_64::_MM_HINT_T0
            )),
            2 => unroll!(2, |i| core::arch::x86_64::_mm_prefetch(
                (p as *const i8).offset(i * 64),
                core::arch::x86_64::_MM_HINT_T0
            )),
            3 => unroll!(3, |i| core::arch::x86_64::_mm_prefetch(
                (p as *const i8).offset(i * 64),
                core::arch::x86_64::_MM_HINT_T0
            )),
            4 => unroll!(4, |i| core::arch::x86_64::_mm_prefetch(
                (p as *const i8).offset(i * 64),
                core::arch::x86_64::_MM_HINT_T0
            )),
            5 => unroll!(5, |i| core::arch::x86_64::_mm_prefetch(
                (p as *const i8).offset(i * 64),
                core::arch::x86_64::_MM_HINT_T0
            )),
            6 => unroll!(6, |i| core::arch::x86_64::_mm_prefetch(
                (p as *const i8).offset(i * 64),
                core::arch::x86_64::_MM_HINT_T0
            )),
            7 => unroll!(7, |i| core::arch::x86_64::_mm_prefetch(
                (p as *const i8).offset(i * 64),
                core::arch::x86_64::_MM_HINT_T0
            )),
            8 => unroll!(8, |i| core::arch::x86_64::_mm_prefetch(
                (p as *const i8).offset(i * 64),
                core::arch::x86_64::_MM_HINT_T0
            )),
            9 => unroll!(9, |i| core::arch::x86_64::_mm_prefetch(
                (p as *const i8).offset(i * 64),
                core::arch::x86_64::_MM_HINT_T0
            )),
            10 => unroll!(10, |i| core::arch::x86_64::_mm_prefetch(
                (p as *const i8).offset(i * 64),
                core::arch::x86_64::_MM_HINT_T0
            )),
            11 => unroll!(11, |i| core::arch::x86_64::_mm_prefetch(
                (p as *const i8).offset(i * 64),
                core::arch::x86_64::_MM_HINT_T0
            )),
            12 => unroll!(12, |i| core::arch::x86_64::_mm_prefetch(
                (p as *const i8).offset(i * 64),
                core::arch::x86_64::_MM_HINT_T0
            )),
            13 => unroll!(13, |i| core::arch::x86_64::_mm_prefetch(
                (p as *const i8).offset(i * 64),
                core::arch::x86_64::_MM_HINT_T0
            )),
            14 => unroll!(14, |i| core::arch::x86_64::_mm_prefetch(
                (p as *const i8).offset(i * 64),
                core::arch::x86_64::_MM_HINT_T0
            )),
            15 => unroll!(15, |i| core::arch::x86_64::_mm_prefetch(
                (p as *const i8).offset(i * 64),
                core::arch::x86_64::_MM_HINT_T0
            )),
            _ => unroll!(16, |i| core::arch::x86_64::_mm_prefetch(
                (p as *const i8).offset(i * 64),
                core::arch::x86_64::_MM_HINT_T0
            )),
        }
    }
}
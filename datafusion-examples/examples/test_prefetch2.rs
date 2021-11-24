#![feature(core_intrinsics)]
#![feature(link_llvm_intrinsics)]
use chrono::prelude::*;
use rand::Rng;
use std::collections::{HashMap, HashSet};
use tonic::server::StreamingService;

use core::arch::x86_64::_mm_prefetch;
use std::arch::x86_64::{_MM_HINT_NTA, _MM_HINT_T0, _MM_HINT_T2};
use std::fs::File;
use std::ops::{Add, Index};
extern crate rand;

use std::intrinsics::prefetch_read_data;
use std::io::{BufReader, Read};
use std::mem;
use std::fs;

#[tokio::main]
async fn main() {
    test2()
}
fn test2() {

    let mut vec_data = Vec::new();
    let mut rng = rand::thread_rng();
    let mut i = 0;
    let size = 100000000;
    while i < size {
        let rag_data = rng.gen::<u64>();
        vec_data.push(rag_data);
        i += 1;
    }

    println!("build data successful ! len : {}",vec_data.len());

    let data_len = vec_data.len();
    let mut ccc = 1;
    while ccc < 50 {
        let dt = Local::now();
        let step = 333;
        let mut i = 0;
        let mut result = 0;
        let start = step*ccc;
        while i < step{
            let mut j = i;
            while j < data_len{
                let k = start + j;
                if k  < data_len {
                    unsafe {
                        //_mm_prefetch(vec_data.as_ptr().offset(k as isize) as *const i8, _MM_HINT_T0)
                        //_mm_prefetch(&vec_data[k] as *const u64 as *const i8, _MM_HINT_T0)
                        prefetch_read_data(vec_data.as_ptr().offset(k as isize),3);
                    }
                }
                result += calcu_normal(vec_data[j as usize]);
                j+=step
            }
            i+=1;
        }

        println!("{}",result);
        println!(
            "usage millis: {} prefetch :{}",
            Local::now().timestamp_millis() - dt.timestamp_millis(),
            ccc
        );
        ccc +=1;
    }
}

fn calcu_heavy (input : u64) -> u64{
    let  val = (input % 99) * (input / 98);
    let d = input as f64 / val as f64;
    d.powf(1999.9) as u64
}

fn calcu_normal (input : u64) -> u64{
    let  val = (input % 99) * (input / 98);
    let n = (((input as f64) * 1.3).sqrt()).sqrt();
    let m = (((val as f64) * 1.3).sqrt()).sqrt();
    (input as f64 * val as f64 * m / 1.1) as u64
}

fn prefetch(p: *const u64) {
    let i8p = p as *const i8;
    unsafe {
        _mm_prefetch(i8p, _MM_HINT_T0);
        //prefetch_read_data(p,3);
    }
}

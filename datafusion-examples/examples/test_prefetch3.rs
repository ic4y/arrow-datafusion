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
// adapted from llvmint generated bindings to remove the dependency

#[tokio::main]
async fn main() {

    let result = fs::read("/Users/liliu/Downloads/presto.tar.gz").unwrap();
    let len = result.len();
    println!("len : {},{}",result.len(),result[0]);



    let dt = Local::now();
    let r = run_withprefetch(result.clone(), len as isize, 512, 8);
    println!("{}",r);
    println!(
        "usage millis: {}",
        Local::now().timestamp_millis() - dt.timestamp_millis()
    );

    // let dt = Local::now();
    // let r = run(result.clone(), len as isize, 512);
    // println!("{}",r);
    // println!(
    //     "usage millis: {}",
    //     Local::now().timestamp_millis() - dt.timestamp_millis()
    // );


}


fn run_withprefetch(data : Vec<u8>, size : isize, step: isize, prefetch: isize) ->isize{
    let mut result = 0;
    println!("run with prefetch{}...",prefetch);
    let mut i = 0;

    while i < step{
        let mut j = i;
        while j < size{
            let k = j + step * prefetch;
            if k  < size {
                unsafe {_mm_prefetch(data.as_ptr().offset(k) as *const i8, _MM_HINT_T0)}
            }
            result += calcu(data[j as usize] as isize);
            j+=step
        }
        i+=1;
    }
    return result;
}

fn run(data : Vec<u8>, size : isize, step: isize) ->isize{
    let mut result = 0;
    println!("run ...");
    let mut i = 0;
    while i < step{
        let mut j = i;
        while j < size{
            result += calcu(data[j as usize] as isize);
            j+=step
        }
        i+=1;
    }
    return result;
}




fn calcu (input : isize) -> isize{
    let  val = (input % 99) * (input / 98);
    let d = input as f64 / val as f64;
    d.powf(1999.9) as isize
}


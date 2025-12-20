
#![allow(unused)]

use std::f32::MIN;

const PI : f32 = 3.14;
fn main() {
    let mut x = 1;
    x += 1;

    // shadowing
    let x : i32 = -1;
    let x : i32 = 2;
    let x : bool = true;

    // Type placeholder  (telling rust to figure out type)
    let  x : _ = 100;

    //constants 
    const NUM : u32 = 2;
    println!("Hello world from example {x}");
    println!("Hello world from example {}",x);

    // positional 
    let square = x * x;
    println!("The square  of {0} * {0} is {1}",x,square);

    // Debug (suppose you want to debug variable x)
    println!("x {:?}",x); // or
    println!("x {:#?}",x); // for complicated type

    // type conversion
    let i : i32 = -1;
    let u : u32 = i as u32;

    // min & max
    let imax = i32::MAX;
    let umin : u32 = u32::MIN;

    //tuples length is known at compile time 
    let t : (u32,bool) = (100,true);

    // access tuple with t.0 
    let (a,b) = t;
    println!("destructing tuple {a} &  {b}");

    
}

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

    // nested tuples
    let tn = ((25,true),(30,3.25,5));
    println!("the value in first index {}", (tn.1).1);

    // unit type
    let tu = ();


    // Array
    let arr: [i32;5] = [0; 5];
    println!("array value si {:?}",arr);
    // particular index access
    println!("element at index 0 is {}",arr[0]);

    // array are immutable
    /// you can mutate it with declairing with mut keyword'
    
    let mut am = [1,2,3];
    am[0] = 10;

    println!("element at 0 in mutable array {}",am[0]);

    for x in am {
        println!("{}",x);
    }

    // loop with index
    for i in 0..am.len() {
        println!("{i} {}",am[i]);
    }

    // enumarate
    for (i,val) in am.iter().enumerate() {
        println!("{} = {}",i,val);
    }

    // slicing with array
    let new_array = [1,2,3,4,5];
    let sarray  = &new_array[2..];

    println!("sliced array is {:?}",sarray);

    // matrix array
    let multi_dem_array: [[i32;3];2] = [[1,2,3],[4,5,6]];

    println!("multi demension array {:?}",multi_dem_array);


    // string and string literals
    let msg:String = String::from("Hello world");
    let msg:String = "Hello world".to_string();

    let s : &str = &msg[0..5];
    println!("The slicing of the msg with string literals {s}");

    // note rust automatically convert &String to &str

    // string interpolation = format
    let lang= "Rust";
    let emoji = "fun";

    let new_string = format!("Hello {} {}",lang,emoji);
 
    
    //Enum

    #[derive(Debug)]
    enum Command {
        Play,
        Stop,
        Skip(u32),
        Back(u32),
        Resize {width:u32,height:u32}
    }

    let cmd:Command = Command::Play;
    let cmd: Command = Command::Skip(5);
    let cmd :Command = Command::Resize{ width: 100, height: 50 };

    println!("{:?}",cmd);

    //Option<T> = Some(T) : None 
    let x  : Option<i32> = Some(1);
    let x : Option<i32> = None;

    //Result<T,E> = Ok(T) | Error(E)
    let x:Result<i32,String> = Ok(100);
    let x : Result<i32, String> = Err("Failed to parse string to number".to_string());    

    // struct data type to group different datatype to single data type
    #[derive(Debug)]
    struct Point {
        x: i32,
        y: i32
    }

    #[derive(Debug)]
    struct Point3D(i32,i32, i32);

    struct Empty;

    #[derive(Debug)]
    struct Circle {
        radius: u32,
        center : Point
    }

    let p = Point{x:1, y:1};
    println!("{:?}",p);
    println!("value of x in point {}",p.x);

    let p = Point3D(-1,0,1);
    println!("{:?}",p);

    let empty = Empty;

    let circle = Circle {
        radius: 3,
        center: Point{x:1,y:1}
    };

    println!("{:?}",circle);

    // shorcuts on struct
    let x = 1;
    let y :i32 = 1;

    let p = Point {x,y}; // no {x:x,y:y}

    // copy fields
    let p0  = Point {x:1,y:1};
    let p1 = Point{x:1,..p0}; // except x copy every thing from p0 means values of all variables

    // Update
    let mut p = Point{x:1,y:1};
    p.x += 1;
    println!("updated p {:#?}",p) 
}
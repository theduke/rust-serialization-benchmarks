#![feature(proc_macro)]
#![feature(step_by)]

extern crate bencher;
extern crate serde;
#[macro_use] extern crate serde_derive;

// Serialization libraries.
extern crate serde_json;
extern crate bincode;

mod data;
mod spec;

use std::io::Write;

use data::*;
use spec::*;

make_tests! {

    serde_json("serde_json") {
        serialize |data| { serde_json::to_string(data).unwrap() }
        deserialize |data| { serde_json::from_str(data).unwrap() }
    }

    bincode("bincode") {
        serialize |data| { 
            bincode::serde::serialize(data, bincode::SizeLimit::Infinite).unwrap() 
        }
        deserialize |data| { 
            bincode::serde::deserialize(data).unwrap() 
        }
    }

}

fn main() {
    println!("Running benchmarks. Let the battles commence!\n");
    let results = run_tests();

    println!("Benchmarks finished. Writing results to ./results.json");
    let json = serde_json::to_string(&results).unwrap();
    let mut f = std::fs::File::create("./results.json").unwrap();
    f.write_all(json.as_bytes()).unwrap();
    f.flush().unwrap();

    println!("Over and out...");
}


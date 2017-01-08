#![feature(proc_macro)]
#![feature(step_by)]

extern crate bencher;
extern crate serde;
#[macro_use] extern crate serde_derive;

// Serialization libraries.
extern crate serde_json;
extern crate bincode;
extern crate serde_cbor;
extern crate serde_yaml;
extern crate rmp_serde;
extern crate toml;
extern crate serde_pickle;
extern crate serde_hjson;
extern crate bson;

mod data;
mod spec;

use std::io::Write;

use data::*;
use spec::*;

make_tests! {

    serde_json_struct("serde_json_struct") {
        serialize |data| { serde_json::to_string(data).unwrap() }
        deserialize |data| { serde_json::from_str(data).unwrap() }
    }

    serde_json_value("serde_json_value") {
        convert_data |data| -> serde_json::Value { serde_json::to_value(data) }
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

    serde_cbor("serde_cbor") {
        serialize |data| { 
            serde_cbor::to_vec(data).unwrap() 
        }
        deserialize |data| { 
            serde_cbor::from_slice(data).unwrap() 
        }
    }
    
    rmp_serde("rmp_serde") {
        serialize |data| { 
            rmp_serde::to_vec(data).unwrap() 
        }
        deserialize |data| { 
            rmp_serde::from_slice(data).unwrap() 
        }
    }
     
    serde_pickle("serde_pickle") {
        serialize |data| { 
            serde_pickle::to_vec(data, true).unwrap() 
        }
        deserialize |data| { 
            serde_pickle::from_slice(data).unwrap() 
        }
    }
     
    serde_yaml("serde_yaml") {
        serialize |data| { 
            serde_yaml::to_string(data).unwrap() 
        }
        deserialize |data| { 
            serde_yaml::from_str(data).unwrap() 
        }
    }

    serde_hjson("serde_hjson") {
        serialize |data| { 
            serde_hjson::to_string(data).unwrap() 
        }
        deserialize |data| { 
            serde_hjson::from_str(data).unwrap() 
        }
    }
    
    /*
    Problem: getting error: "attempted conversion of invalid data type"
    bson("bson") {
        serialize |data| { 
            let value = bson::to_bson(data).unwrap();
            let doc = value.to_extended_document();
            let mut buf = Vec::new();
            bson::encode_document(&mut buf, &doc).unwrap();
            buf
        }
        deserialize |data| { 
            let raw_doc = bson::decode_document(&mut std::io::Cursor::new(&data[..])).unwrap();
            let doc = bson::Bson::Document(raw_doc);
            bson::from_bson(doc).unwrap() 
        }
    }

    toml("toml") {
        serialize |data| { 
            toml::encode_str(data).unwrap() 
        }
        deserialize |data| { 
            toml::decode_str(data).unwrap() 
        }
    }
    */
    
}

fn main() {
    println!("Running benchmarks. Let the battles commence!\n");
    let results = run_tests();
    println!("\n\n");

    println!("Benchmarks finished. Writing results to ./results.json");
    let json = serde_json::to_string(&results).unwrap();
    let mut f = std::fs::File::create("./results.json").unwrap();
    f.write_all(json.as_bytes()).unwrap();
    f.flush().unwrap();

    println!("Over and out...");
}


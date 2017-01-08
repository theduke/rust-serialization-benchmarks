# Rust Serialization Benchmarks

This is a rust crate that compares the performance of various Rust serialization 
libraries.

## Libraries

* [serde_json](https://github.com/serde-rs/json)
* [bincode](https://github.com/TyOverby/bincode)
* [serde_cbor](https://github.com/pyfisch/cbor);
* [rmp_serde](https://github.com/3Hren/msgpack-rust)
* [serde_yaml](https://github.com/dtolnay/serde-yaml)
* [serde_pickle](https://github.com/birkenfeld/serde-pickle)
* [serde_hjson](https://github.com/hjson/hjson-rust)

## Results

I'll put some pretty graphs and analysis here soon.

Note that this benchmark also tests formats for which speed is not really a big
concern, like toml and yaml. They are just included for reference.

## How To Run The Benchmarks

Running the benchmarks is easy, as long as you have rust nightly availbe 
(preferably via rustup).

```bash
git clone https://github.com/theduke/rust-serialization-benchmarks.git 
cd rust-serialization-benchmarks
rustup override set nightly
cargo run --release
```

This will both print the benchmark results to stdout, and write a `results.json` 
file that contains detailed results as JSON.

Please note that the release build can take quite a while...

## Test Setup

Just a few notes regarding test setup and fairness:

Each library is tested for serialization and deserialization performance with
different variations of data that you can check out in src/data.rs: 

* StaticData: a 'static' struct without any pointers/allocated data
* DynamicData: a struct that contains allocated data (vectors and maps)
* NestedData: a complicated nested struct with static and dynamic data

The code really just calls the respective serialize/deserialize functions 
without checking correctness of the result, just success.

Most tests serialize directly from the struct to the serialization format via
serde. 
Some other tests serialize to the libraries respective value type instead. 

The  [bencher]() library is used, which is a fork of the libtest crate contained
in rustc (used for the built-in 
[benchmark tests](https://doc.rust-lang.org/book/benchmark-tests.html).

It runs test code until variance stabilizes
and the cache can assumed to be warm. Then the test is run multiple times and
some statistical variables are computed (variance, stddev, mean, median).

I'm using a fork that only contains the trivial change of making a struct field
public to get at more detailed information than just the median.

The *mb per second* data point is calculated by taking the byte length of the 
serialized data and extrapolating from the *median*.

## How To Add Another Library

If you want to add another library to the benchmark, I'm happy about pull 
requests.

Adding another lib is easy:

* Add the library as dependency in Cargo.toml (prefer * for version)
* Add `extern crate my-crate` in src/main.rs under the other libraries
* Add the test to to the make_tests! macro in src/main.rs.
    It should be pretty straight forward how to add a test.
    Note that if you are not adding a library that serializes directly to/from
    a struct with serde, you have to specify a convert_data block. 
    Check the serde_json_value{} specification for an example.


#![macro_use]

pub use std::collections::BTreeMap;

use bencher;

#[derive(Clone, Debug, Serialize)]
pub struct Result {
    pub ns_avg: f64,    
    pub ns_median: f64,
    pub ns_variance: f64,
    pub ns_std_dev: f64,
    pub mb_per_sec: f64,
}

impl Result {
    pub fn from_samples(s: bencher::BenchSamples, byte_size: usize) -> Self {
        let mb_per_sec = (1000000000.0 / s.ns_iter_summ.mean) * (byte_size as f64 / 1000000.0);
        Result {
            ns_avg: s.ns_iter_summ.mean,    
            ns_median: s.ns_iter_summ.median,
            ns_variance: s.ns_iter_summ.var,
            ns_std_dev: s.ns_iter_summ.std_dev,
            mb_per_sec: mb_per_sec,
        }
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct CaseResult {
    pub name: String,
    pub serialize: Result,
    pub deserialize: Result,
    pub data: Vec<u8>,
}

#[derive(Clone, Debug, Serialize)]
pub struct LibraryResult {
    pub library: String,
    pub cases: BTreeMap<String, CaseResult>,
}

macro_rules! test_variant {
    {
        variant($variant_name:expr, $data_type:ty, $creater:expr) {
            serialize |$ser_data:ident| { $($ser_code:tt)* }
            deserialize |$deser_data:ident| { $($deser_code:tt)* }
        }
    } => {

        {
            println!("    Variant {}:", $variant_name);
            let data = $creater;

            // Serialize data to get a byte count. 
            let serialized_result = {
                let $ser_data = &data;
                $($ser_code)*
            };
            let byte_len = serialized_result.len();
            
            // Serialize.
            let ser_samples = bencher::bench::benchmark(|bench| {
                let $ser_data = &data;
                bench.iter(|| {
                    let res = { $($ser_code)* };
                    res
                });
            });

            // Deserialize.
            let deser_samples = bencher::bench::benchmark(|bench| {
                let $deser_data = &serialized_result;
                //let mut target = serialized_result.clone();
                bench.iter(|| {
                    let res: $data_type = { $($deser_code)* };
                    res
                });
            });
            
            let res = CaseResult {
                name: $variant_name.to_string(),
                serialize: Result::from_samples(ser_samples, byte_len),
                deserialize: Result::from_samples(deser_samples, byte_len),
                data: (&serialized_result).clone().into(),
            };
            println!("        Serialize: median: {}ns / mb/sec: {}",
                     res.serialize.ns_median,
                     res.serialize.mb_per_sec);
            println!("        Deserialize: median: {}ns / mb/sec: {}",
                     res.deserialize.ns_median,
                     res.deserialize.mb_per_sec);
            res
        }

    };

    {
        variant($variant_name:expr, $data_type:ty, $creater:expr) {
            convert_data |$convert_data:ident| -> $converted_type:ty { $($convert_code:tt)* }
            serialize |$ser_data:ident| { $($ser_code:tt)* }
            deserialize |$deser_data:ident| { $($deser_code:tt)* }
        }
    } => {

        {
            println!("    Variant {}:", $variant_name);
            let data = $creater;
            // Convert data.
            let data: $converted_type = {
                let $convert_data = data;
                $( $convert_code )*
            };

            // Serialize data to get a byte count. 
            let serialized_result = {
                let $ser_data = &data;
                $($ser_code)*
            };
            let byte_len = serialized_result.len();
            
            // Serialize.
            let ser_samples = bencher::bench::benchmark(|bench| {
                let $ser_data = &data;
                bench.iter(|| {
                    let res = { $($ser_code)* };
                    res
                });
            });

            // Deserialize.
            let deser_samples = bencher::bench::benchmark(|bench| {
                let $deser_data = &serialized_result;
                //let mut target = serialized_result.clone();
                bench.iter(|| {
                    let res: $converted_type = { $($deser_code)* };
                    res
                });
            });
            
            let res = CaseResult {
                name: $variant_name.to_string(),
                serialize: Result::from_samples(ser_samples, byte_len),
                deserialize: Result::from_samples(deser_samples, byte_len),
                data: (&serialized_result).clone().into(),
            };
            println!("        Serialize: median: {}ns / mb/sec: {}",
                     res.serialize.ns_median,
                     res.serialize.mb_per_sec);
            println!("        Deserialize: median: {}ns / mb/sec: {}",
                     res.deserialize.ns_median,
                     res.deserialize.mb_per_sec);
            res
        }

    }
}

#[macro_export]
macro_rules! make_tests {
    {
        $(
            $library:ident($name:expr) {
                $( $code:tt )*
            }
        )*
    } => {

        fn run_tests() -> Vec<::spec::LibraryResult> {
            let mut results = Vec::new();
            
            {
                $(
                    println!("##########\nTesting {}:", $name);
                    let mut lib_result = LibraryResult {
                        library: $name.to_string(),    
                        cases: BTreeMap::new(),
                    };

                    let res = test_variant! {
                        variant("static", StaticData, StaticData::new()) { $( $code )* }
                    };
                    lib_result.cases.insert("static".into(), res);

                    let res = test_variant! {
                        variant("dynamic", DynamicData, DynamicData::new()) { $( $code )* }
                    };
                    lib_result.cases.insert("dynamic".into(), res);

                    let res = test_variant! {
                        variant("nested", NestedData, NestedData::new()) { $( $code )* }
                    };
                    lib_result.cases.insert("nested".into(), res);

                    results.push(lib_result);
                    println!("##########\n");
                )*
            }

            results
        }
    }
}


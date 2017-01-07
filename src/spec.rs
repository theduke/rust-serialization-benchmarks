#![macro_use]

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
    pub serialize: Result,
    pub deserialize: Result,
}

#[derive(Clone, Debug, Serialize)]
pub struct LibraryResult {
    pub library: String,
    pub static_data: CaseResult,
    pub dynamic_data: CaseResult,
    pub nested_data: CaseResult,
}

#[macro_export]
macro_rules! make_tests {
    {
        $(
            $library:ident($name:expr) {
                serialize |$ser_data:ident| { $($ser_code:tt)* }
                deserialize |$deser_data:ident| { $($deser_code:tt)* }
            }
        )*
    } => {

        fn run_tests() -> Vec<::spec::LibraryResult> {
            let mut results = Vec::new();

            $(
                {
                    println!("##########\nTesting {}:", $name);
                    
                    // STATIC data.
                    println!("  Static data:");
                    let static_case = {
                        
                        let data = build_static_data();
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
                                $($ser_code)*
                            });
                        });

                        // Deserialize.
                        let deser_samples = bencher::bench::benchmark(|bench| {
                            let $deser_data = &serialized_result.clone();
                            bench.iter(|| {
                                let _: StaticData = { $($deser_code)* };
                            });
                        });

                        CaseResult {
                            serialize: Result::from_samples(ser_samples, byte_len),
                            deserialize: Result::from_samples(deser_samples, byte_len),
                        }
                    };
                    println!("    Serialize: median: {}ns / mb/sec: {}",
                             static_case.serialize.ns_median,
                             static_case.serialize.mb_per_sec);
                    println!("    Deserialize: median: {}ns / mb/sec: {}",
                             static_case.deserialize.ns_median,
                             static_case.deserialize.mb_per_sec);

                    // Dynamic data.
                    println!("  Dynamic data:");
                    let dynamic_case = {
                        
                        let data = build_dynamic_data();
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
                                $($ser_code)*
                            });
                        });

                        // Deserialize.
                        let deser_samples = bencher::bench::benchmark(|bench| {
                            let $deser_data = &serialized_result.clone();
                            bench.iter(|| {
                                let _: DynamicData = { $($deser_code)* };
                            });
                        });

                        CaseResult {
                            serialize: Result::from_samples(ser_samples, byte_len),
                            deserialize: Result::from_samples(deser_samples, byte_len),
                        }
                    };
                    println!("    Serialize: median: {}ns / mb/sec: {}",
                             dynamic_case.serialize.ns_median,
                             dynamic_case.serialize.mb_per_sec);
                    println!("    Deserialize: median: {}ns / mb/sec: {}",
                             dynamic_case.deserialize.ns_median,
                             dynamic_case.deserialize.mb_per_sec);

                    // Nested data.
                    println!("  Nested data:");
                    let nested_case = {
                        let data = build_nested_data();
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
                                $($ser_code)*
                            });
                        });

                        // Deserialize.
                        let deser_samples = bencher::bench::benchmark(|bench| {
                            let $deser_data = &serialized_result;
                            bench.iter(|| {
                                let _: NestedData = { $($deser_code)* };
                            });
                        });

                        CaseResult {
                            serialize: Result::from_samples(ser_samples, byte_len),
                            deserialize: Result::from_samples(deser_samples, byte_len),
                        }
                    };
                    println!("    Serialize: median: {}ns / mb/sec: {}",
                             nested_case.serialize.ns_median,
                             nested_case.serialize.mb_per_sec);
                    println!("    Deserialize: median: {}ns / mb/sec: {}",
                             nested_case.deserialize.ns_median,
                             nested_case.deserialize.mb_per_sec);

                    let lib_res = LibraryResult {
                        library: $name.to_string(),
                        static_data: static_case,
                        dynamic_data: dynamic_case,
                        nested_data: nested_case,
                    };
                    results.push(lib_res);
                    println!("##########\n");
                }
            )*
    
            results
        }
    }
}


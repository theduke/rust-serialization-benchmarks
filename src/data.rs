use std;
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StaticData {
    v_bool: bool,
    v_i8: i8,
    v_i16: i16,
    v_i32: i32,
    v_i64: i64,
    v_u8: u8,
    v_u16: u16,
    v_u32: u32,
    // Not all libraries support u64, so I'm skipping it.
    //v_u64: u64,
    v_f32: f32,
    v_f64: f64, 
    v_char_ascii: char,
    v_char_unicode: char,
    v_i32_none: Option<i32>,
    v_i32_some: Option<i32>,
}

impl StaticData {
    pub fn new() -> Self {
        StaticData {
            v_bool: true,
            v_i8: std::i8::MAX,
            v_i16: std::i16::MAX,
            v_i32: std::i32::MAX,
            v_i64: std::i64::MAX,
            v_u8: std::u8::MAX,
            v_u16: std::u16::MAX,
            v_u32: std::u32::MAX,
            //v_u64: std::u64::MAX,
            v_f32: std::f32::MAX,
            v_f64: std::f64::MAX, 
            v_char_ascii: 'x',
            v_char_unicode: 'Ö',
            v_i32_none: None,
            v_i32_some: Some(std::i32::MAX),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DynamicData {
    v_static: StaticData,
    v_str_empty: String,
    v_str_short: String,
    v_str_long: String,
    v_vec_empty: Vec<i32>,
    v_vec_short: Vec<i32>,
    v_vec_long: Vec<i32>,
    v_map_empty: HashMap<String, i32>,
    v_map: HashMap<String, i32>,
}

impl DynamicData {
    pub fn new() -> Self {
        DynamicData {
            v_static: StaticData::new(),
            v_str_empty: "".into(),
            v_str_short: "abcdeÖ".into(),
            v_str_long: (1..20).fold(String::new(), |s, _| s + "abcdefÖ"),
            v_vec_empty: vec![],
            v_vec_short: vec![0, 1000, std::i32::MAX],
            v_vec_long: (0..10000).step_by(100).collect(),
            v_map_empty: HashMap::new(),
            v_map: (1..20).fold(HashMap::<String, i32>::new(), |mut m, i| {
                m.insert(format!("key_{}", i), 1000); 
                m 
            }),
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct NestedData {
    v_data: DynamicData,
    v_list: Vec<DynamicData>,
    v_map: HashMap<String, DynamicData>,
}

impl NestedData {
    pub fn new() -> Self {
        let data = DynamicData::new();
        NestedData {
            v_data: data.clone(),
            v_list: (0..3).map(|_| data.clone()).collect(),
            v_map: (0..3).fold(HashMap::<String, DynamicData>::new(), |mut m, i| {
                m.insert(format!("key_{}", i), data.clone());
                m
            }),
        }
    }
}


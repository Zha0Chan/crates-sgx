
//#![feature(trace_macros)]



use nom::{character::is_alphanumeric, number::complete::recognize_float};

use std::collections::HashMap;
use std::str;
use crates_unittest::test_case;
use std::prelude::v1::*;
#[derive(Debug, PartialEq)]
pub enum JsonValue {
  Str(String),
  Num(f32),
  Array(Vec<JsonValue>),
  Object(HashMap<String, JsonValue>),
}

named!(float<f32>, flat_map!(recognize_float, parse_to!(f32)));

//FIXME: verify how json strings are formatted
named!(
  string<&str>,
  delimited!(
    char!('"'),
    //map_res!(escaped!(call!(alphanumeric), '\\', is_a!("\"n\\")), str::from_utf8),
    map_res!(
      escaped!(take_while1!(is_alphanumeric), '\\', one_of!("\"n\\")),
      str::from_utf8
    ),
    char!('"')
  )
);

named!(
  array<Vec<JsonValue>>,
  delimited!(
    char!('['),
    separated_list0!(char!(','), value),
    char!(']')
  )
);

named!(
  key_value<(&str, JsonValue)>,
  separated_pair!(string, char!(':'), value)
);

named!(
  hash<HashMap<String, JsonValue>>,
  map!(
    delimited!(
      char!('{'),
      separated_list0!(char!(','), key_value),
      char!('}')
    ),
    |tuple_vec| {
      let mut h: HashMap<String, JsonValue> = HashMap::new();
      for (k, v) in tuple_vec {
        h.insert(String::from(k), v);
      }
      h
    }
  )
);

named!(
  value<JsonValue>,
  alt!(
    hash   => { |h|   JsonValue::Object(h)            } |
    array  => { |v|   JsonValue::Array(v)             } |
    string => { |s|   JsonValue::Str(String::from(s)) } |
    float  => { |num| JsonValue::Num(num)             }
  )
);

#[test_case]
fn json_object() {
  let input = r#"{"a":42,"b":"x"}\0"#;

  let mut expected_map = HashMap::new();
  expected_map.insert(String::from("a"), JsonValue::Num(42f32));
  expected_map.insert(String::from("b"), JsonValue::Str(String::from("x")));
  let expected = JsonValue::Object(expected_map);

  assert_eq!(expected, value(input.as_bytes()).unwrap().1);
}

#[test_case]
fn json_array() {
  let input = r#"[42,"x"]\0"#;

  let expected_vec = vec![JsonValue::Num(42f32), JsonValue::Str(String::from("x"))];
  let expected = JsonValue::Array(expected_vec);

  assert_eq!(expected, value(input.as_bytes()).unwrap().1);
}

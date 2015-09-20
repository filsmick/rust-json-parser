use std::collections::HashMap;

#[derive(PartialEq, Debug)]
pub enum JsonValue<'a> {
  String(&'a str),
  Number(f64),
  Boolean(bool),
  Object(HashMap<&'a str, JsonValue<'a>>),
  Array(Vec<JsonValue<'a>>),
  Null
}

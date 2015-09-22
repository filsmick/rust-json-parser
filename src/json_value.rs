use std::collections::HashMap;
use std::borrow::Cow;

#[derive(PartialEq, Debug)]
pub enum JsonValue<'a> {
  String(Cow<'a, str>),
  Number(f64),
  Boolean(bool),
  Object(HashMap<Cow<'a, str>, JsonValue<'a>>),
  Array(Vec<JsonValue<'a>>),
  Null
}

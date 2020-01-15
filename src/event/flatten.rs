use crate::event::{Atom, LogEvent, ValueKind};
use serde_json::{Map, Value};

/// Recursevly inserts json values to event
pub fn flatten(event: &mut LogEvent, map: Map<String, Value>) {
    for (name, value) in map {
        insert(event, name, value);
    }
}

/// Recursevly inserts json values to event under given name
pub fn insert<K: Into<Atom> + AsRef<str>>(event: &mut LogEvent, name: K, value: Value) {
    match value {
        Value::String(string) => {
            event.insert(name, string);
        }
        Value::Number(number) => {
            let val: ValueKind = if let Some(val) = number.as_i64() {
                val.into()
            } else if let Some(val) = number.as_f64() {
                val.into()
            } else {
                number.to_string().into()
            };

            event.insert(name, val);
        }
        Value::Bool(b) => {
            event.insert(name, b);
        }
        Value::Null => {
            event.insert(name, "");
        }
        Value::Array(array) => {
            for (i, element) in array.into_iter().enumerate() {
                let element_name = format!("{}[{}]", name.as_ref(), i);
                insert(event, element_name, element);
            }
        }
        Value::Object(object) => {
            for (key, value) in object.into_iter() {
                let item_name = format!("{}.{}", name.as_ref(), key);
                insert(event, item_name, value);
            }
        }
    }
}

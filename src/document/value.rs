use std::collections::HashMap;

use serde::ser::{Serialize, Serializer, SerializeMap, SerializeSeq};
use serde_derive::{Deserialize};

use crate::Rule;

#[derive(Deserialize, Debug, Clone)]
pub enum Value {
    Boolean(bool),
    Number(f64),
    String(String),
    Array(Vec<Value>),
    Map(HashMap<String, Value>),
}

impl Serialize for Value {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Value::Boolean(value) => serializer.serialize_bool(*value),
            Value::Number(value) => serializer.serialize_f64(*value),
            Value::String(value) => serializer.serialize_str(value),
            Value::Array(value) => {
                let mut seq = serializer.serialize_seq(Some(value.len()))?;

                for value in value {
                    seq.serialize_element(value)?;
                }

                seq.end()
            }
            Value::Map(value) => {
                let mut map = serializer.serialize_map(Some(value.len()))?;

                for (key, value) in value {
                    map.serialize_entry(key, value)?;
                }

                map.end()
            }
        }
    }
}

impl Value {
    pub fn from_pair(pair: pest::iterators::Pair<Rule>) -> Option<Value> {
        let value_pair = match pair.as_rule() {
            Rule::value => pair.into_inner().next()?.into_inner().next()?,
            Rule::collection => pair.into_inner().next()?,
            Rule::literal => pair.into_inner().next()?,
            Rule::boolean => pair,
            Rule::number => pair,
            Rule::string => pair,
            Rule::array => pair,
            Rule::map => pair,
            _ => return None,
        };

        match value_pair.as_rule() {
            Rule::boolean => Some(Value::Boolean(match value_pair.into_inner().next()?.as_rule() {
                Rule::boolean_true => true,
                Rule::boolean_false => false,
                _ => return None,
            })),
            Rule::number => {
                let inner = value_pair.into_inner().next()?;
                let raw = inner.as_str();

                Some(Value::Number(match inner.as_rule() {
                    Rule::number_bin => i32::from_str_radix(raw, 2).ok()? as f64,
                    Rule::number_hex => i32::from_str_radix(raw, 16).ok()? as f64,
                    Rule::numer_decimal => raw.parse().ok()?,
                    _ => return None,
                }))
            }
            Rule::string => {
                let inner = value_pair.into_inner().next()?;

                match inner.as_rule() {
                    Rule::string_inner => Some(Value::String(inner.as_str().to_string())),
                    _ => None
                }
            }
            Rule::array => {
                let mut values = Vec::new();

                for pair in value_pair.into_inner() {
                    values.push(Value::from_pair(pair)?);
                }

                Some(Value::Array(values))
            }
            Rule::map => {
                let mut values = HashMap::new();

                for pair in value_pair.into_inner() {
                    if pair.as_rule() != Rule::map_pair {
                        continue;
                    }

                    let mut inner = pair.into_inner();
                    let key_pair = inner.next()?.into_inner().next()?;
                    let value_pair = inner.next()?;

                    let key = match key_pair.as_rule() {
                        Rule::string => match Value::from_pair(key_pair)? {
                            Value::String(s) => s,
                            _ => return None,
                        },
                        Rule::identifier => key_pair.as_str().to_string(),
                        _ => return None,
                    };

                    values.insert(key, Value::from_pair(value_pair)?);
                }

                Some(Value::Map(values))
            }
            _ => None,
        }
    }
}
use serde::ser::{SerializeSeq, Serializer};
use serde_derive::{Serialize, Deserialize};

use crate::{Value, Rule};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Statement {
    #[serde(rename = "$name")]
    pub identifier: String,

    #[serde(rename = "$args")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub arguments: Vec<Value>,

    #[serde(rename = "$block")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block: Option<Block>,
}

impl Statement {
    pub fn from_pair(pair: pest::iterators::Pair<Rule>) -> Option<Statement> {
        if pair.as_rule() != Rule::statement {
            return None;
        }

        let mut inner = pair.into_inner();

        let identifier = inner.next()?.as_str().to_string();
        let mut arguments = Vec::new();
        let mut block = None;

        for pair in inner {
            match pair.as_rule() {
                Rule::block => block = Some(Block::from_pair(pair)?),
                _ => arguments.push(Value::from_pair(pair)?),
            }
        }

        Some(Statement {
            identifier,
            arguments,
            block,
        })
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct Block {
    pub attributes: Vec<Statement>,
}

impl serde::Serialize for Block {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(self.attributes.len()))?;

        for value in &self.attributes {
            seq.serialize_element(value)?;
        }

        seq.end()
    }
}

impl Block {
    pub fn from_pair(pair: pest::iterators::Pair<Rule>) -> Option<Block> {
        if pair.as_rule() != Rule::block {
            return None;
        }

        let mut attributes = Vec::new();

        for pair in pair.into_inner() {
            attributes.push(Statement::from_pair(pair)?);
        }

        Some(Block { attributes })
    }
}
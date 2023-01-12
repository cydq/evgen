use serde::ser::{Serialize, Serializer, SerializeSeq};
use serde_derive::{Deserialize};

use crate::{Statement, Rule};

#[derive(Deserialize, Debug, Clone)]
pub struct Document {
    pub statements: Vec<Statement>,
}

impl Serialize for Document {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(self.statements.len()))?;

        for value in &self.statements {
            seq.serialize_element(value)?;
        }

        seq.end()
    }
}

impl Document {
    pub fn from_pair(pair: pest::iterators::Pair<Rule>) -> Option<Document> {
        if pair.as_rule() != Rule::document {
            return None;
        }
        
        let mut statements = Vec::new();
        
        for pair in pair.into_inner() {
            if pair.as_rule() == Rule::EOI {
                break;
            }

            statements.push(Statement::from_pair(pair)?);
        }
        
        Some(Document { statements })
    }
}
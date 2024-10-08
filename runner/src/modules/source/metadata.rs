use serde::{Deserialize, Deserializer, Serialize};

use crate::modules::lang::Lang;

/// A structure defining a data type.
#[derive(Debug, Deserialize, Serialize)]
pub struct Typ {
    pub initial: String,
    pub transformed: String,
    pub form: Form,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub enum Form {
    Unit,
    Array,
    Pointer,
}

/// A structure defining the name and type for a variable.
#[derive(Debug, Deserialize)]
pub struct Variable {
    pub name: String,
    #[serde(rename = "type")]
    pub typ: Typ,
}

/// A structure defining the metadata associated to a question.
#[derive(Debug)]
pub struct MetaData {
    pub name: String,
    pub params: Vec<Variable>,
    pub return_type: Typ,
    pub cleanup: bool,
}

impl<'de> Deserialize<'de> for MetaData {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[derive(Deserialize)]
        struct PreMetaData {
            lang: Lang,
            name: String,
            params: Vec<PreVariable>,
            #[serde(rename = "return")]
            return_type: Return,
        }

        #[derive(Deserialize)]
        struct PreVariable {
            name: String,
            #[serde(rename = "type")]
            typ: String,
        }

        #[derive(Deserialize)]
        struct Return {
            #[serde(rename = "type")]
            typ: String,
        }

        let pre_metadata = PreMetaData::deserialize(deserializer)?;
        let return_type = pre_metadata
            .lang
            .parse(&pre_metadata.return_type.typ)
            .unwrap();
        let mut cleanup = return_type.form == Form::Pointer;

        Ok(MetaData {
            name: pre_metadata.name,
            params: pre_metadata
                .params
                .iter()
                .map(|v| {
                    let typ = pre_metadata.lang.parse(&v.typ).unwrap();
                    cleanup |= typ.form == Form::Pointer;

                    Variable { name: v.name.clone(), typ }
                })
                .collect(),
            return_type,
            cleanup,
        })
    }
}

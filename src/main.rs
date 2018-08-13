#![feature(rust_2018_preview, use_extern_macros)]
#![warn(rust_2018_idioms)]

#[macro_use]
extern crate failure;

use failure::Fail;
use serde;
use serde_derive::{Deserialize, Serialize};

mod evaluator;
mod gui;

const ALGORITHM: u32 = 0;
const VALUE: u32 = 1;

#[derive(Serialize, Deserialize, Debug)]
pub struct TokenizedFile {
    text: Vec<TextChunk>,
    algorithms: Vec<AlgorithmToken>,
    values: Vec<ValueToken>,
}

impl TokenizedFile {
    fn find_value_by_id(&self, id: u32) -> Option<&ValueToken> {
        for ep in self.values.iter() {
            if ep.id == id {
                return Some(&ep);
            }
        }
        None
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct TextChunk {
    ty: u32,
    text: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct AlgorithmToken {
    loc: u32,
    ty: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct ValueToken {
    loc: u32,
    ty: String,
    id: u32,
    val: i32,
}

fn deserialize() -> Result<TokenizedFile, failure::Error> {
    let j = r#"
    {
        "text": [
            {
                "ty": 1,
                "text": "The sum of "
            },
            {
                "ty": 1,
                "text": " and "
            },
            {
                "ty": 0,
                "text": " is "
            },
            {
                "ty": 2,
                "text": "."
            }
        ],
        "algorithms": [
            {
                "loc": 10,
                "ty": [
                    "add :0 :1"
                ]
            }
        ],
        "values": [
            {
                "loc": 31,
                "ty": "number",
                "id": 0,
                "val": 3
            },
            {
                "loc": 36,
                "ty": "number",
                "id": 1,
                "val": 10
            }
        ]
    }
    "#;

    let t: TokenizedFile = serde_json::from_str(j)?;
    Ok(t)
}

fn main() -> Result<(), failure::Error> {
    let mut file = deserialize()?;
    let mut evaluations = evaluator::evaluate(&file).unwrap();
    let text = gui::build_html_output(&mut file, &mut evaluations);

    gui::run(text);
    Ok(())
}

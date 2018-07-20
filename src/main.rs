extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate failure;

mod evaluator;

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

fn build_output(text: &mut TokenizedFile, evaluations: &mut Vec<evaluator::EvalResult>) -> String {
    let mut string = Vec::new();
    
    for t in text.text.iter() {
        match t.ty {
            ALGORITHM => {
                string.push(t.text.clone());
                string.push(evaluations.pop().unwrap().to_string());
            },
            VALUE => {
                string.push(t.text.clone());
                string.push(text.values.pop().unwrap().val.to_string());
            }
            _ => {
                string.push(t.text.clone());
            }
        }
    }
    string.join("")
}

fn main() -> Result<(), failure::Error> {
    let mut file = deserialize()?;
    let mut evaluations = evaluator::evaluate(&file).unwrap();
    println!("build_output = {:?}", build_output(&mut file, &mut evaluations));
    Ok(())
}

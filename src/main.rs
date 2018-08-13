#![feature(rust_2018_preview, use_extern_macros)]
#![warn(rust_2018_idioms)]

#[macro_use]
extern crate failure;

use failure::Fail;
use serde;
use serde_derive::{Deserialize, Serialize};
use web_view::*;

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

#[allow(non_camel_case_types)]
#[derive(Deserialize)]
#[serde(tag = "cmd")]
enum Cmd {
    init,
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
                string.push(format!(r#"<span class="evaluation">"#));
                string.push(evaluations.pop().unwrap().to_string());
                string.push(r#"</span>"#.to_string());
            }
            VALUE => {
                let v = text.values.pop().unwrap();
                string.push(t.text.clone());
                string.push(format!(r#"<span class="value" id="{}">"#, v.id));
                string.push(v.val.to_string());
                string.push(r#"</span>"#.to_string());
            }
            _ => {
                string.push(t.text.clone());
            }
        }
    }
    string.join("")
}

fn render<'a, T>(webview: &mut WebView<'a, T>, state: &str) {
    webview.eval(&format!("rpc.render({})", serde_json::to_string(state).unwrap()));
}

fn main() -> Result<(), failure::Error> {
    let mut file = deserialize()?;
    let mut evaluations = evaluator::evaluate(&file).unwrap();
    
	let text = build_output(&mut file, &mut evaluations);
    
    let html = format!(
	r#"<!doctype html>
	<html>
		<head>
		{style}
		</head>
		<body>
		{script}
		<div id="content"></div>
		</body>
	</html>"#, style = inline_style(include_str!("gui/style.css")), script = inline_script(include_str!("gui/app.js"))
	);

	println!("{}", &html);
    let size = (800, 600);
    let resizeable = true;
    let debug = true;
    let init_cb = |webview: MyUnique<WebView<String>>| {};
    let userdata = text;
    run(
        "Test",
        Content::Html(html),
        Some(size),
        resizeable,
        debug,
        init_cb,
        |webview, arg, state: &mut String| {
            match serde_json::from_str(arg).unwrap() {
                Cmd::init => (),
            }
            render(webview, state);
        },
        userdata,
    );

    Ok(())
}

fn inline_style(s: &str) -> String {
    format!(r#"<style>{}</style>"#, s)
}

fn inline_script(s: &str) -> String {
    format!(r#"<script type="text/javascript">{}</script>"#, s)
}


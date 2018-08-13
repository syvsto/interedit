use serde_derive::Deserialize;
use serde_json;
use web_view::*;

use crate::{evaluator, TokenizedFile};

const ALGORITHM: u32 = 0;
const VALUE: u32 = 1;

#[allow(non_camel_case_types)]
#[derive(Deserialize)]
#[serde(tag = "cmd")]
enum Cmd {
    init,
}
fn render<'a, T>(webview: &mut WebView<'a, T>, state: &str) {
    webview.eval(&format!(
        "rpc.render({})",
        serde_json::to_string(state).expect("Couldn't eval render json")
    ));
}

pub fn build_html_output(
    text: &mut TokenizedFile,
    evaluations: &mut Vec<evaluator::EvalResult>,
) -> String {
    let mut string = Vec::new();

    for t in text.text.iter() {
        match t.ty {
            ALGORITHM => {
                string.push(t.text.clone());
                string.push(format!(r#"<span class="evaluation">"#));
                string.push(
                    evaluations
                        .pop()
                        .expect("Couldn't get evaluation")
                        .to_string(),
                );
                string.push(r#"</span>"#.to_string());
            }
            VALUE => {
                let v = text.values.pop().expect("Couldn't get expression");
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

fn inline_style(s: &str) -> String {
    format!(r#"<style>{}</style>"#, s)
}

fn inline_script(s: &str) -> String {
    format!(r#"<script type="text/javascript">{}</script>"#, s)
}

pub fn run(text: String) {
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
	</html>"#,
        style = inline_style(include_str!("gui/style.css")),
        script = inline_script(include_str!("gui/app.js"))
    );

    //	println!("{}", &html);
    let size = (800, 600);
    let resizeable = true;
    let debug = true;
    let init_cb = |webview: MyUnique<WebView<String>>| {};
    let userdata = text;
    web_view::run(
        "Test",
        Content::Html(html),
        Some(size),
        resizeable,
        debug,
        init_cb,
        |webview, arg, state: &mut String| {
            match serde_json::from_str(arg).expect("Couldn't parse args") {
                Cmd::init => (),
            }
            render(webview, state);
        },
        userdata,
    );
}

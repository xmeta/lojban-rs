use js_sys::Date;
use lojban::{classify_word, friendly_error, parse, tree, word_stats, Rule};
use pest::error::{Error, ErrorVariant, InputLocation, LineColLocation};
use wasm_bindgen::prelude::*;

const MAX_REGRESSION_CASES: usize = 200;

#[wasm_bindgen]
pub fn parse_text(input: &str) -> String {
    parse_response(input)
}

#[wasm_bindgen]
pub fn regression_text(input: &str) -> String {
    regression_response(input)
}

fn parse_response(input: &str) -> String {
    let stats = word_stats(input);
    let started = Date::now();
    let parsed = parse(input);
    let elapsed_ms = Date::now() - started;
    let stats_json = format!(
        "{{\"tokens\":{},\"gismu\":{},\"lujvo\":{},\"fuivla\":{},\"cmevla\":{},\"cmavo\":{},\"unknown\":{}}}",
        stats.tokens, stats.gismu, stats.lujvo, stats.fuivla,
        stats.cmevla, stats.cmavo, stats.unknown
    );
    match parsed {
        Ok(pairs) => {
            let ast = tree::to_json(pairs.clone());
            let pretty = tree::to_json_pretty(pairs.clone());
            let tree_text = tree::to_tree_string(pairs.clone());
            let sexpr = tree::to_sexpr(pairs.clone());
            let leaves = tree::leaf_spans(pairs);
            let leaves_json = leaves.iter().map(leaf_json).collect::<Vec<_>>().join(",");
            format!(
                "{{\"ok\":true,\"elapsed_ms\":{elapsed_ms:.3},\"stats\":{stats_json},\"ast\":{ast},\"pretty\":{},\"tree\":{},\"sexpr\":{},\"leaves\":[{leaves_json}]}}",
                json_string(&pretty),
                json_string(&tree_text),
                json_string(&sexpr)
            )
        }
        Err(error) => {
            let details = error_details_json(&error);
            format!(
                "{{\"ok\":false,\"elapsed_ms\":{elapsed_ms:.3},\"stats\":{stats_json},\"error\":{},\"details\":{details}}}",
                json_string(&friendly_error(&error))
            )
        }
    }
}
fn regression_response(input: &str) -> String {
    let batch_started = Date::now();
    let mut cases = Vec::new();
    let mut passed = 0usize;
    let mut failed = 0usize;
    let mut processed = 0usize;
    let mut truncated = false;

    for (index, raw_line) in input.lines().enumerate() {
        let text = raw_line.trim_end_matches('\r');
        if text.trim().is_empty() {
            continue;
        }
        if processed >= MAX_REGRESSION_CASES {
            truncated = true;
            break;
        }
        processed += 1;
        let started = Date::now();
        let parsed = parse(text);
        let elapsed_ms = Date::now() - started;
        match parsed {
            Ok(_) => {
                passed += 1;
                cases.push(format!(
                    "{{\"line\":{},\"text\":{},\"ok\":true,\"elapsed_ms\":{elapsed_ms:.3}}}",
                    index + 1,
                    json_string(text)
                ));
            }
            Err(error) => {
                failed += 1;
                let details = error_details_json(&error);
                cases.push(format!(
                    "{{\"line\":{},\"text\":{},\"ok\":false,\"elapsed_ms\":{elapsed_ms:.3},\"error\":{},\"details\":{details}}}",
                    index + 1,
                    json_string(text),
                    json_string(&friendly_error(&error))
                ));
            }
        }
    }

    let total = passed + failed;
    let elapsed_ms = Date::now() - batch_started;
    format!(
        "{{\"total\":{total},\"passed\":{passed},\"failed\":{failed},\"elapsed_ms\":{elapsed_ms:.3},\"truncated\":{truncated},\"cases\":[{}]}}",
        cases.join(",")
    )
}

fn error_details_json(error: &Error<Rule>) -> String {
    let (start, end) = match error.location {
        InputLocation::Pos(pos) => (pos, pos),
        InputLocation::Span((start, end)) => (start, end),
    };
    let (line, column) = match error.line_col {
        LineColLocation::Pos((line, column)) => (line, column),
        LineColLocation::Span((line, column), _) => (line, column),
    };
    let expected = match &error.variant {
        ErrorVariant::ParsingError { positives, .. } => positives
            .iter()
            .take(12)
            .map(|rule| json_string(&format!("{rule:?}")))
            .collect::<Vec<_>>()
            .join(","),
        ErrorVariant::CustomError { .. } => String::new(),
    };
    format!(
        "{{\"start\":{start},\"end\":{end},\"line\":{line},\"column\":{column},\"expected\":[{expected}]}}"
    )
}

fn leaf_json(leaf: &tree::LeafSpan) -> String {
    let bare = leaf.text.trim_start_matches(['.', ',', '!', '?']);
    format!(
        "{{\"rule\":{},\"text\":{},\"start\":{},\"end\":{},\"class\":{}}}",
        json_string(&format!("{:?}", leaf.rule)),
        json_string(&leaf.text),
        leaf.start,
        leaf.end,
        json_string(classify_word(bare))
    )
}

fn json_string(value: &str) -> String {
    let mut out = String::with_capacity(value.len() + 2);
    out.push('"');
    for ch in value.chars() {
        match ch {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if c < ' ' => {
                use std::fmt::Write as _;
                let _ = write!(out, "\\u{:04x}", c as u32);
            }
            c => out.push(c),
        }
    }
    out.push('"');
    out
}

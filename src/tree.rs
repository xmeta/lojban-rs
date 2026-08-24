//! 解析木の文字列化ユーティリティ
//!
//! pest の解析結果([`Pairs`])を S 式または整形ツリーへ変換する。
//! `EOI` は構造上のマーカーであるため出力から除外する。

use std::fmt::Write;

use pest::iterators::{Pair, Pairs};

use crate::grammar::Rule;

/// S 式形式で出力する。
///
/// 葉ノードは `(rule "text")`、内部ノードは `(rule (child) ...)` の形式。
///
/// ```
/// use lojban::{grammar::{LojbanParser, Rule}, tree};
/// use pest::Parser;
///
/// let pairs = LojbanParser::parse(Rule::text, "mi klama").unwrap();
/// let s = tree::to_sexpr(pairs);
/// assert!(s.starts_with("(text"), "{s}");
/// ```
pub fn to_sexpr(pairs: Pairs<'_, Rule>) -> String {
    let mut out = String::new();
    for pair in pairs {
        write_pair(pair, &mut out);
    }
    out
}

fn visible_children<'a>(pair: &'a Pair<'a, Rule>) -> Vec<Pair<'a, Rule>> {
    pair.clone()
        .into_inner()
        .filter(|p| p.as_rule() != Rule::EOI)
        .collect()
}

fn write_pair(pair: Pair<'_, Rule>, out: &mut String) {
    let children = visible_children(&pair);
    if children.is_empty() {
        let _ = write!(out, "({:?} {})", pair.as_rule(), quote(pair.as_str()));
    } else {
        let _ = write!(out, "({:?}", pair.as_rule());
        for child in children {
            out.push(' ');
            write_pair(child, out);
        }
        out.push(')');
    }
}

/// インデント付き整形ツリーで出力する。
///
/// 各行は `rule_name: "テキスト"` 形式。
///
/// ```
/// use lojban::{grammar::{LojbanParser, Rule}, tree};
/// use pest::Parser;
///
/// let pairs = LojbanParser::parse(Rule::text, "mi klama").unwrap();
/// let s = tree::to_tree_string(pairs);
/// assert!(s.contains("text:"), "{s}");
/// ```
pub fn to_tree_string(pairs: Pairs<'_, Rule>) -> String {
    let mut out = String::new();
    for pair in pairs {
        write_tree(pair, 0, &mut out);
    }
    out
}

fn write_tree(pair: Pair<'_, Rule>, depth: usize, out: &mut String) {
    if pair.as_rule() == Rule::EOI {
        return;
    }
    let indent = "  ".repeat(depth);
    let _ = writeln!(out, "{}{:?}: {:?}", indent, pair.as_rule(), pair.as_str());
    for inner in pair.into_inner() {
        write_tree(inner, depth + 1, out);
    }
}

/// 解析木を JSON 文字列として出力する。
///
/// 各ノードは `{"rule": "...", "text": "...", "children": [...]}` 形式。
///
/// ```
/// use lojban::{grammar::{LojbanParser, Rule}, tree};
/// use pest::Parser;
///
/// let pairs = LojbanParser::parse(Rule::text, "mi klama").unwrap();
/// let j = tree::to_json(pairs);
/// assert!(j.starts_with("{\"version\":1,\"rule\":\"text\""), "{j}");
/// ```
pub fn to_json(pairs: Pairs<'_, Rule>) -> String {
    let mut out = String::new();
    for pair in pairs {
        write_json(&pair, &mut out);
    }
    // ルートオブジェクトにスキーマ版数を埋め込む(v0.39 以降)
    if out.starts_with('{') {
        out.insert_str(1, "\"version\":1,");
    }
    out
}

fn write_json(pair: &Pair<'_, Rule>, out: &mut String) {
    if pair.as_rule() == Rule::EOI {
        return;
    }
    let _ = write!(
        out,
        "{{\"rule\":\"{:?}\",\"text\":{}",
        pair.as_rule(),
        json_escape(pair.as_str())
    );
    let children = visible_children(pair);
    if !children.is_empty() {
        out.push_str(",\"children\":[");
        for (i, c) in children.iter().enumerate() {
            if i > 0 {
                out.push(',');
            }
            write_json(c, out);
        }
        out.push(']');
    }
    out.push('}');
}

/// JSON 文字列リテラルとして安全な形で出力する。
/// Graphviz DOT 形式で解析木を出力する。
///
/// ノードは `rule` 名と原文をラベルに持つ。`dot -Tsvg` 等で可視化できる。
///
/// ```
/// use lojban::{parse, tree};
///
/// let pairs = parse("mi klama").unwrap();
/// let dot = tree::to_dot(pairs);
/// assert!(dot.starts_with("digraph parse"), "{dot}");
/// assert!(dot.contains("KOhA_core"), "{dot}");
/// ```
pub fn to_dot(pairs: Pairs<'_, Rule>) -> String {
    let mut out = String::from("digraph parse {\n");
    out.push_str("  node [shape=box fontname=\"monospace\"];\n");
    let mut counter = 0usize;
    for pair in pairs {
        if pair.as_rule() == Rule::EOI {
            continue;
        }
        let id = write_dot(&pair, &mut out, &mut counter);
        let _ = id;
    }
    out.push('}');
    out
}

fn write_dot(pair: &Pair<'_, Rule>, out: &mut String, counter: &mut usize) -> usize {
    let id = *counter;
    *counter += 1;
    let text = dot_escape(pair.as_str());
    let _ = writeln!(out, "  n{id} [label=\"{:?}\\n{text}\"];", pair.as_rule());
    for child in visible_children(pair) {
        let cid = write_dot(&child, out, counter);
        let _ = writeln!(out, "  n{id} -> n{cid};");
    }
    id
}

fn dot_escape(s: &str) -> String {
    s.replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\l")
}

/// スタンドアロン HTML 文書として解析木を出力する。
///
/// 内部ノードは `<details>`/`<summary>` による折りたたみ付きで、
/// スタイルは文書内に同梱される。ブラウザで開くだけで解析木を閲覧できる。
///
/// ```
/// use lojban::{parse, tree};
///
/// let pairs = parse("mi klama").unwrap();
/// let html = tree::to_html(pairs);
/// assert!(html.starts_with("<!DOCTYPE html>"), "{html}");
/// assert!(html.contains("<details"), "{html}");
/// assert!(html.contains("rule-KOhA_core"), "{html}");
/// ```
pub fn to_html(pairs: Pairs<'_, Rule>) -> String {
    const HEAD: &str = r#"<!DOCTYPE html>
<html lang="ja">
<head>
<meta charset="utf-8">
<title>lojban parse tree</title>
<style>
body { font-family: ui-monospace, monospace; margin: 1em; background: #fafafa; }
ul.tree, ul { padding-left: 1.2em; list-style: none; }
details > summary { cursor: pointer; list-style: revert; }
code { background: #eee; padding: 0 3px; border-radius: 3px; }
.t { color: #555; }
</style>
</head>
<body>
"#;
    let mut out = String::from(HEAD);
    out.push_str("<ul class=\"tree\">\n");
    for pair in pairs {
        if pair.as_rule() == Rule::EOI {
            continue;
        }
        write_html(&pair, &mut out, 0);
    }
    out.push_str("</ul>\n</body>\n</html>\n");
    out
}

fn write_html(pair: &Pair<'_, Rule>, out: &mut String, depth: usize) {
    let rule = format!("{:?}", pair.as_rule());
    let text = html_escape(pair.as_str());
    let children = visible_children(pair);
    // 深さ 0〜1 は初期展開、それより深い節は折りたたみ
    let open = if depth <= 1 { " open" } else { "" };
    if children.is_empty() {
        let _ = writeln!(
            out,
            "<li><code class=\"rule-{rule}\" title=\"{text}\">{rule}</code> <span class=\"t\">{text}</span></li>"
        );
        return;
    }
    let _ = writeln!(
        out,
        "<li><details{open}><summary><code class=\"rule-{rule}\">{rule}</code> <span class=\"t\">{text}</span></summary>"
    );
    out.push_str("<ul>\n");
    for child in children {
        write_html(&child, out, depth + 1);
    }
    out.push_str("</ul>\n</details></li>\n");
}

fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

fn json_escape(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 2);
    out.push('"');
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if c.is_control() => {
                let _ = write!(out, "\\u{{{:04x}}}", c as u32);
            }
            c => out.push(c),
        }
    }
    out.push('"');
    out
}

/// インデント付き整形ツリーで出力する。
fn quote(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 2);
    out.push('"');
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if c.is_control() => {
                let _ = write!(out, "\\u{{{:x}}}", c as u32);
            }
            c => out.push(c),
        }
    }
    out.push('"');
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::grammar::{LojbanParser, Rule};
    use pest::Parser;

    #[test]
    fn sexpr_単語() {
        let input = "mi";
        let pairs = LojbanParser::parse(Rule::word, input).unwrap();
        let s = to_sexpr(pairs);
        assert_eq!(s, "(word (CMAVO_clause (CMAVO_core \"mi\")))");
    }

    #[test]
    fn tree_単語() {
        let input = "gerku";
        let pairs = LojbanParser::parse(Rule::word, input).unwrap();
        let s = to_tree_string(pairs);
        assert!(s.contains("word: \"gerku\""));
        assert!(s.contains("BRIVLA_clause"), "{s}");
        assert!(!s.contains("EOI"));
    }

    #[test]
    fn json_単語() {
        let input = "mi";
        let pairs = LojbanParser::parse(Rule::word, input).unwrap();
        let j = to_json(pairs);
        assert!(j.contains("\"rule\":\"word\""), "{j}");
        assert!(j.contains("CMAVO_clause"), "{j}");
        assert!(!j.contains("EOI"), "{j}");
    }

    #[test]
    fn quote_エスケープ() {
        assert_eq!(quote("a\"b\\c\nd"), "\"a\\\"b\\\\c\\nd\"");
    }
}

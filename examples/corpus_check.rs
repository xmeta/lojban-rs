//! コーパス検査ハーネス: 1行1文のファイルを読みみ、各文の解析可否を報告する
//!
//! 使い方: cargo run --example corpus_check -- /path/to/corpus.txt
use lojban::grammar::{LojbanParser, Rule};
use pest::Parser;

fn main() {
    let path = std::env::args().nth(1).expect("usage: corpus_check <file>");
    let text = std::fs::read_to_string(&path).expect("cannot read file");
    let (mut ok, mut ng) = (0u32, 0u32);
    let mut failures = Vec::new();
    for line in text.lines() {
        let s = line.trim();
        if s.is_empty() || s.starts_with('#') {
            continue;
        }
        match LojbanParser::parse(Rule::text, s) {
            Ok(_) => ok += 1,
            Err(e) => {
                ng += 1;
                failures.push(format!("{s}\n    -> {}", e.variant.message()));
            }
        }
    }
    println!("pass: {ok}  fail: {ng}  total: {}", ok + ng);
    if !failures.is_empty() {
        println!("\n=== failures ===");
        for f in &failures {
            println!("{f}");
        }
    }
}

//! 規則別の所要時間分離(一時ハーネス)
use std::time::Instant;

use lojban::grammar::{LojbanParser, Rule};
use pest::Parser;

fn chain(chars: usize) -> String {
    let base = "kaprenmit";
    let reps = chars / base.len();
    let mut s = base.repeat(reps);
    s.push_str(&base[..chars - s.len()]);
    s
}

fn main() {
    let s = chain(40);
    let rules = [
        ("zifcme", Rule::zifcme),
        ("jbocme", Rule::jbocme),
        ("cmavo_form", Rule::cmavo_form),
        ("cmavo", Rule::cmavo),
        ("gismu", Rule::gismu),
        ("CVV_final_rafsi", Rule::CVV_final_rafsi),
        ("fuhivla", Rule::fuhivla),
        ("brivla", Rule::brivla),
        ("CVCy_lujvo", Rule::CVCy_lujvo),
    ];
    for (name, rule) in rules {
        let t = Instant::now();
        let r = LojbanParser::parse(rule, &s);
        println!(
            "{name:>18} {:>10.2?} {}",
            t.elapsed(),
            if r.is_ok() { "ok" } else { "err" }
        );
    }
}

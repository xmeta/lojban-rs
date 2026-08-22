//! ロジバン文法(pest 定義)
//!
//! 文法本体は [`lojban.pest`](./lojban.pest) に記述する。
//! zantufa-1.9999.peg(guskant/gerna_cipra)を参考に移植する。

use pest_derive::Parser;

/// ロジバン文法のパーサー(pest_derive により生成)。
#[derive(Parser)]
#[grammar = "src/grammar/lojban.pest"]
pub struct LojbanParser;

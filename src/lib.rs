//! ロジバン(Lojban)PEG パーサー(pest ベース)
//!
//! 文法は [`grammar`] モジュールの `lojban.pest` に定義し、
//! zantufa-1.9999.peg(guskant/gerna_cipra)を参考に移植する。

pub mod grammar;
pub mod tree;

use pest::error::Error;
use pest::iterators::Pairs;
use pest::Parser;

pub use grammar::{LojbanParser, Rule};

/// ロジバンテキストを解析する。
///
/// 成功時は解析木([`Pairs`])を返す。失敗時は行・列・期待要素を含む
/// [`Error`] を返す。
pub fn parse(text: &str) -> Result<Pairs<'_, Rule>, Error<Rule>> {
    LojbanParser::parse(Rule::text, text)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn 正常な文() {
        assert!(parse("").is_ok());
        assert!(parse("mi tavla do").is_ok());
        assert!(parse("le mlatu cu cadzu").is_ok());
        // 項だけのフラグメントも正しい発話(zantufa の fragment 準拠)
        assert!(parse("mi").is_ok());
        // 時制詞を含む文
        assert!(parse("mi pu klama do").is_ok());
    }

    #[test]
    fn 不正な入力は拒否() {
        // "q" はロジバンのアルファベットに存在しない
        assert!(parse("q").is_err());
        // 未知の語形
        assert!(parse("qqq").is_err());
        // 空の描述
        assert!(parse("le cu").is_err());
    }
}

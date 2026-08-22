//! ロジバン(Lojban)PEG パーサー(pest ベース)
//!
//! 文法は [`grammar`] モジュールの `lojban.pest` に定義し、
//! zantufa-1.9999.peg(guskant/gerna_cipra)を参考に移植する。

pub mod grammar;
pub mod tree;

use std::borrow::Cow;

use pest::error::{Error, ErrorVariant};
use pest::iterators::Pairs;
use pest::{Parser, Span};

pub use grammar::{LojbanParser, Rule};

/// ロジバンテキストを解析する。
///
/// 成功時は解析木([`Pairs`])を返す。失敗時は行・列・期待要素を含む
/// [`Error`] を返す。
///
/// ZOI 引用(`zoi DELIM 本文 DELIM`)は純粋な PEG では扱えないため、
/// 解析前に本文を `zo'e` へ置換する正規化を行う。この場合の解析木は
/// 元テキストではなく正規化テキストに基づく(ZOI を含む入力のみ)。
/// 返値のライフタイム制約上、正規化テキストは意図的にリークして
/// プロセス終了まで保持される(ZOI を含む入力のみ・1回あたり入力長分)。
pub fn parse(text: &str) -> Result<Pairs<'_, Rule>, Error<Rule>> {
    let normalized = normalize_zoi(text).map_err(|msg| {
        let end = text.len();
        Error::new_from_span(
            ErrorVariant::CustomError { message: msg },
            Span::new(text, 0, end).unwrap_or_else(|| Span::new(text, 0, 0).unwrap()),
        )
    })?;
    match normalized {
        Cow::Borrowed(s) => LojbanParser::parse(Rule::text, s),
        Cow::Owned(s) => {
            let leaked: &'static str = Box::leak(s.into_boxed_str());
            LojbanParser::parse(Rule::text, leaked)
        }
    }
}

/// ZOI 引用(`zoi DELIM 本文 DELIM`)の事前スキャン。
///
/// 区切り語の対応を検証し、本文を `zo'e` に置換した正規化テキストを返す。
/// ZOI がなければ元テキストをそのまま返す([`Cow::Borrowed`])。
/// 未閉鎖の場合はエラーメッセージを返す。直前の語が `zo`(単語引用)の
/// 場合は ZOI として扱わない。
fn normalize_zoi(text: &str) -> Result<Cow<'_, str>, String> {
    let b = text.as_bytes();

    // 空白区切りのトークン位置を収集
    let mut spans: Vec<(usize, usize)> = Vec::new();
    let mut i = 0usize;
    while i < b.len() {
        while i < b.len() && b[i].is_ascii_whitespace() {
            i += 1;
        }
        if i >= b.len() {
            break;
        }
        let start = i;
        while i < b.len() && !b[i].is_ascii_whitespace() {
            i += 1;
        }
        spans.push((start, i));
    }

    let mut out: Option<String> = None;
    let mut copied = 0usize; // ここまで text からコピー済み
    let mut k = 0usize;
    while k < spans.len() {
        let (s, e) = spans[k];
        let token = &text[s..e];
        let prev_is_zo = k > 0 && text[spans[k - 1].0..spans[k - 1].1].eq_ignore_ascii_case("zo");
        if !token.eq_ignore_ascii_case("zoi") || prev_is_zo || k + 1 >= spans.len() {
            k += 1;
            continue;
        }
        // 区切り語
        let (ds, de) = spans[k + 1];
        let delim = &text[ds..de];
        // 同一トークンの再出現を探す
        let close = spans[k + 2..]
            .iter()
            .position(|&(a, c)| &text[a..c] == delim)
            .map(|off| spans[k + 2 + off]);
        let (cs, ce) = match close {
            Some(p) => p,
            None => {
                return Err(format!(
                    "未閉鎖の zoi 引用です(区切り語 {delim:?} が再出現しません)"
                ))
            }
        };
        let o = out.get_or_insert_with(|| String::with_capacity(text.len()));
        o.push_str(&text[copied..s]);
        o.push_str(&text[s..de]); // zoi DELIM
        o.push_str(" zo'e "); // 本文を正規化
        o.push_str(delim); // 閉じ区切り
        copied = ce;
        k += spans[k + 2..].iter().position(|&(a, _)| a == cs).unwrap() + 3;
    }
    match out {
        Some(mut o) => {
            o.push_str(&text[copied..]);
            Ok(Cow::Owned(o))
        }
        None => Ok(Cow::Borrowed(text)),
    }
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

    #[test]
    fn zoi_引用() {
        assert!(parse("mi cusku zoi .ky. hello world .ky.").is_ok());
        assert!(parse("zoi gy English text gy").is_ok());
        // 未閉鎖
        assert!(parse("mi cusku zoi .ky. abc").is_err());
        // 区切り語不一致
        assert!(parse("zoi .ky. abc .bz.").is_err());
        // zo 単語引用の対象としての zoi は影響なし
        assert!(parse("zo zoi cu cmavo").is_ok());
    }
}

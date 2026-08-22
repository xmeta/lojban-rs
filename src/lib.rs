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
///
/// # Examples
///
/// ```
/// use lojban::parse;
///
/// let pairs = parse("mi tavla do").unwrap();
/// assert!(pairs.count() >= 1);
/// ```
pub fn parse(text: &str) -> Result<Pairs<'_, Rule>, Error<Rule>> {
    let err = |msg: String| {
        let end = text.len();
        Error::new_from_span(
            ErrorVariant::CustomError { message: msg },
            Span::new(text, 0, end).unwrap_or_else(|| Span::new(text, 0, 0).unwrap()),
        )
    };
    check_nesting(text).map_err(err)?;
    fn leak_parse(s: String) -> Result<Pairs<'static, Rule>, Error<Rule>> {
        let leaked: &'static str = Box::leak(s.into_boxed_str());
        LojbanParser::parse(Rule::text, leaked)
    }
    let normalized = normalize_zoi(text).map_err(err)?;
    let erased = apply_erasure(normalized.as_ref()).map_err(err)?;
    match erased {
        Cow::Borrowed(_) => match normalized {
            Cow::Borrowed(s) => LojbanParser::parse(Rule::text, s),
            Cow::Owned(s) => leak_parse(s),
        },
        Cow::Owned(s) => leak_parse(s),
    }
}

/// 入れ子深度の上限。PEG のバックトラックが引用・括弧の深い入れ子で
/// 指数時間になるため、リソース保護として受理を打ち切る。
const MAX_NEST: i32 = 8;

/// 引用(lu / lohu)と数式括弧(vei)の入れ子深度を検査する。
///
/// 上限超過の場合はエラー(深い入れ子は解析が指数時間になるため、
/// 高速な拒否に切り替える)。閉じ過ぎ(負の深さ)は文法側の判定に委ねる。
fn check_nesting(text: &str) -> Result<(), String> {
    let mut lu: i32 = 0;
    let mut lohu: i32 = 0;
    let mut vei: i32 = 0;
    for tok in text.split_ascii_whitespace() {
        match tok
            .trim_start_matches(['.', ',', '!', '?'])
            .to_ascii_lowercase()
            .as_str()
        {
            "lu" => lu += 1,
            "li'u" | "lihu" => lu -= 1,
            "lo'u" | "lohu" => lohu += 1,
            "le'u" | "lehu" => lohu -= 1,
            "vei" => vei += 1,
            "ve'o" | "veho" => vei -= 1,
            _ => {}
        }
        if lu > MAX_NEST {
            return Err(format!("lu 引用の入れ子が深すぎます(上限 {MAX_NEST})"));
        }
        if lohu > MAX_NEST {
            return Err(format!("lo'u 引用の入れ子が深すぎます(上限 {MAX_NEST})"));
        }
        if vei > MAX_NEST {
            return Err(format!("vei 括弧の入れ子が深すぎます(上限 {MAX_NEST})"));
        }
    }
    Ok(())
}

/// 消去語(si / su)の意味論的処理。
///
/// - `si`: 直前の語を消去(文区切りは跨がない、連続可)
/// - `su`: 直前の文区切り(`.i …` / `ni'o` / `niho`)まで遡って消去
///
/// 引用(lu…li'u、lohu…lehu)内の si/su は内容として保護し、
/// `zo` の直後の語も保護する。消去が行われた場合は語を空白区切りで
/// 再構成した正規化テキストを返す(解析木は消去後に基づく)。
fn apply_erasure(text: &str) -> Result<Cow<'_, str>, String> {
    let b = text.as_bytes();
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

    // 前置のポーズ記号(.)等を除いた語形
    fn bare(tok: &str) -> &str {
        tok.trim_start_matches(['.', ',', '!', '?'])
    }
    let is_sep = |t: &str| matches!(bare(t).to_ascii_lowercase().as_str(), "i" | "niho" | "ni'o");

    let mut keep = vec![true; spans.len()];
    let mut lu_depth: i32 = 0;
    let mut lohu_depth: i32 = 0;
    let mut prev_zo = false;
    let mut changed = false;

    for k in 0..spans.len() {
        let t = &text[spans[k].0..spans[k].1];
        let bb = bare(t).to_ascii_lowercase();
        if lu_depth > 0 || lohu_depth > 0 {
            match bb.as_str() {
                "lu" => lu_depth += 1,
                "li'u" | "lihu" => lu_depth -= 1,
                "lo'u" | "lohu" => lohu_depth += 1,
                "le'u" | "lehu" => lohu_depth -= 1,
                _ => {}
            }
            prev_zo = false;
            continue;
        }
        match bb.as_str() {
            "lu" => lu_depth += 1,
            "lo'u" | "lohu" => lohu_depth += 1,
            "zo" => prev_zo = true,
            _ => {
                let protected = prev_zo;
                prev_zo = false;
                if protected {
                    continue;
                }
                if bb == "si" {
                    let mut j = k;
                    while j > 0 {
                        j -= 1;
                        if keep[j] && !is_sep(&text[spans[j].0..spans[j].1]) {
                            keep[j] = false;
                            break;
                        }
                    }
                    keep[k] = false;
                    changed = true;
                } else if bb == "su" {
                    for j in (0..k).rev() {
                        if is_sep(&text[spans[j].0..spans[j].1]) {
                            break;
                        }
                        keep[j] = false;
                    }
                    keep[k] = false;
                    changed = true;
                }
            }
        }
    }

    if !changed {
        return Ok(Cow::Borrowed(text));
    }
    let mut out = String::with_capacity(text.len());
    for (idx, alive) in keep.iter().enumerate() {
        if *alive {
            if !out.is_empty() {
                out.push(' ');
            }
            out.push_str(&text[spans[idx].0..spans[idx].1]);
        }
    }
    Ok(Cow::Owned(out))
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
    fn 消去_si() {
        use crate::tree::to_sexpr;
        let pairs = parse("mi klama si cadzu").unwrap();
        let s = to_sexpr(pairs);
        assert!(!s.contains("\"klama\""), "{s}");
        assert!(s.contains("\"cadzu\""), "{s}");
        // 連続 si
        let pairs = parse("mi klama do si si cadzu").unwrap();
        let s = to_sexpr(pairs);
        assert!(!s.contains("\"do\""), "{s}");
        assert!(s.contains("\"cadzu\""), "{s}");
    }

    #[test]
    fn 消去_su() {
        use crate::tree::to_sexpr;
        // 直前の .i まで遡って消去(.i は残る)
        let pairs = parse("mi klama .i do su tavla").unwrap();
        let s = to_sexpr(pairs);
        assert!(!s.contains("\"do\""), "{s}");
        assert!(s.contains("\"tavla\""), "{s}");
    }

    #[test]
    fn 引用内とzo直後の消去語は保護される() {
        use crate::tree::to_sexpr;
        // 引用内の si は内容として残る
        let pairs = parse("lu mi klama si li'u cu melbi").unwrap();
        let s = to_sexpr(pairs);
        assert!(s.contains("\"klama\""), "{s}");
        assert!(s.contains("\"si\""), "{s}");
        // zo の引用対象としての si
        assert!(parse("mi cusku zo si").is_ok());
    }

    #[test]
    fn 入れ子深度上限() {
        // 上限以内は受理される
        assert!(parse("lu lu lu mi klama li'u li'u li'u").is_ok());
        // 上限超過は指数時間を避けるため高速に拒否される
        let deep = format!("{}{}", "lu ".repeat(20), "li'u ".repeat(20));
        assert!(parse(&deep).is_err());
        let deep_vei = format!("li {}pa{}", "vei ".repeat(20), " ve'o".repeat(20));
        assert!(parse(&deep_vei).is_err());
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

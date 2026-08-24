//! ロジバン PEG パーサー CLI
//!
//! モード: 解析(5出力形式) / --lines 行単位バッチ / --stats 語種統計 /
//! --classify 語種判定 / --build-lujvo / --split-lujvo。
//! 各モードの挙動は tests/cli.rs のエンドツーエンドテストが保証する。

use std::io::Read;
use std::process::ExitCode;

use clap::Parser as ClapParser;

use lojban::grammar::Rule;
use lojban::tree;
use lojban::{lujvo, LojbanParser};
use pest::Parser;

#[derive(ClapParser, Debug)]
#[command(name = "lojban", version, about = "ロジバン PEG パーサー")]
struct Args {
    /// 解析するロジバンテキスト(未指定なら stdin または --file)
    text: Option<String>,
    /// ロジバンテキストを読むファイル
    #[arg(short = 'f', long)]
    file: Option<String>,
    /// S 式形式で出力する
    #[arg(long)]
    sexpr: bool,
    /// JSON 形式で出力する
    #[arg(long)]
    json: bool,
    /// Graphviz DOT 形式で出力する
    #[arg(long)]
    dot: bool,
    /// HTML(入れ子リスト)形式で出力する
    #[arg(long)]
    html: bool,
    /// 出力せず成否のみで終了する(バッチ検証用。エラーは stderr に出る)
    #[arg(short = 'q', long)]
    quiet: bool,
    /// 入力を行単位で個別解析する(1行 = 1文。失敗行は行番号付きで報告)
    #[arg(long)]
    lines: bool,
    /// 語種別のトークン統計を出力する(解析成否に依存しない)
    #[arg(long)]
    stats: bool,
    /// 語種を判定する(gismu / lujvo / fu'ivla / cmevla / cmavo / unknown)
    #[arg(long)]
    classify: Option<String>,
    /// lujvo を生成する(rafsi を空白またはカンマ区切りで指定)
    #[arg(long)]
    build_lujvo: Option<String>,
    /// lujvo を rafsi 列に分解して表示
    #[arg(long)]
    split_lujvo: Option<String>,
}

fn main() -> ExitCode {
    let args = Args::parse();

    if let Some(raw) = &args.classify {
        return run_classify(raw, args.json);
    }
    if let Some(word) = &args.split_lujvo {
        return run_split_lujvo(word, args.json);
    }
    if let Some(spec) = &args.build_lujvo {
        return run_build_lujvo(spec, args.json);
    }

    let Some(input) = resolve_input(&args) else {
        return ExitCode::from(2);
    };
    if args.stats {
        return run_stats(&input);
    }
    if args.lines {
        return run_lines(&args, &input);
    }
    run_parse(&args, &input)
}

/// 入力の解決(優先順位: 位置引数 > -f > stdin)。
/// stdin が空で使い方の表示が必要な場合は None を返す。
fn resolve_input(args: &Args) -> Option<String> {
    if let Some(t) = &args.text {
        return Some(t.clone());
    }
    if let Some(path) = &args.file {
        return match std::fs::read_to_string(path) {
            Ok(s) => Some(s),
            Err(e) => {
                eprintln!("エラー: {path} を読めませんでした: {e}");
                None
            }
        };
    }
    let mut buf = String::new();
    if std::io::stdin().read_to_string(&mut buf).is_err() {
        eprintln!("エラー: stdin の読み込みに失敗しました");
        return None;
    }
    if buf.trim().is_empty() {
        eprintln!("使い方: lojban [テキスト] [--sexpr]");
        return None;
    }
    Some(buf)
}

/// --classify: 語種判定(単語/複数語、平文/JSON)
fn run_classify(raw: &str, json: bool) -> ExitCode {
    // 空白・カンマ区切りで複数語に対応
    let words: Vec<&str> = raw
        .split(|c: char| c.is_ascii_whitespace() || c == ',')
        .map(|w| w.trim_start_matches(['.', ',', '!', '?']))
        .filter(|w| !w.is_empty())
        .collect();
    let classes: Vec<(&str, &str)> = words.iter().map(|w| (*w, classify_word(w))).collect();
    if json {
        let items: Vec<String> = classes
            .iter()
            .map(|(w, c)| format!("{{\"word\":\"{w}\",\"class\":\"{c}\"}}"))
            .collect();
        if items.len() == 1 {
            println!("{}", items[0]);
        } else {
            println!("[{}]", items.join(","));
        }
    } else {
        for (w, c) in &classes {
            println!("{w}: {c}");
        }
    }
    ExitCode::SUCCESS
}

/// --split-lujvo: lujvo を rafsi 列へ分解(平文/JSON)
fn run_split_lujvo(word: &str, json: bool) -> ExitCode {
    match lujvo::decompose(word) {
        Ok(parts) => {
            if json {
                let items: Vec<String> = parts
                    .iter()
                    .map(|p| match p {
                        lujvo::Part::Rafsi { text, form } => format!(
                            "{{\"kind\":\"rafsi\",\"text\":\"{text}\",\"form\":\"{form:?}\"}}"
                        ),
                        lujvo::Part::Hyphen(c) => {
                            format!("{{\"kind\":\"hyphen\",\"char\":\"{c}\"}}")
                        }
                    })
                    .collect();
                println!("{{\"word\":\"{word}\",\"parts\":[{}]}}", items.join(","));
            } else {
                for p in parts {
                    match p {
                        lujvo::Part::Rafsi { text, form } => println!("{text} ({form:?})"),
                        lujvo::Part::Hyphen(c) => println!("-{c}- [hyphen]"),
                    }
                }
            }
            ExitCode::SUCCESS
        }
        Err(e) => {
            eprintln!("エラー: {e}");
            ExitCode::from(1)
        }
    }
}

/// --build-lujvo: rafsi 列から lujvo を生成(平文/JSON)
fn run_build_lujvo(spec: &str, json: bool) -> ExitCode {
    let parts: Vec<&str> = spec
        .split(|c: char| c == ',' || c.is_ascii_whitespace())
        .filter(|s| !s.is_empty())
        .collect();
    match lujvo::build(&parts) {
        Ok(built) => {
            if json {
                let forms: Vec<String> = built
                    .forms
                    .iter()
                    .map(|f| format!("{:?}", format!("{f:?}")))
                    .collect();
                println!(
                    "{{\"word\":\"{}\",\"score\":{},\"hyphens\":{},\"forms\":[{}]}}",
                    built.word,
                    built.score(),
                    built.hyphens,
                    forms.join(",")
                );
            } else {
                println!("{} (score {})", built.word, built.score());
            }
            ExitCode::SUCCESS
        }
        Err(e) => {
            eprintln!("エラー: {e}");
            ExitCode::from(1)
        }
    }
}

/// --stats: 語種別トークン統計(JSON)。解析成功時は文数も付与
fn run_stats(input: &str) -> ExitCode {
    let mut tally = std::collections::BTreeMap::new();
    let mut total = 0usize;
    for tok in input
        .split(|c: char| c.is_ascii_whitespace() || matches!(c, '.' | ',' | '!' | '?'))
        .filter(|w| !w.is_empty())
    {
        total += 1;
        *tally.entry(classify_word(tok)).or_insert(0usize) += 1;
    }
    let g = |k: &str| tally.get(k).copied().unwrap_or(0);
    // 解析が成功する場合のみ文数を付与(失敗時はフィールド自体を省略)
    let sentences = lojban::parse(input)
        .ok()
        .map(|pairs| count_sentences(&mut pairs.into_iter()));
    match sentences {
        Some(n) => println!(
            "{{\"tokens\":{},\"sentences\":{},\"gismu\":{},\"lujvo\":{},\"fu'ivla\":{},\"cmevla\":{},\"cmavo\":{},\"unknown\":{}}}",
            total, n, g("gismu"), g("lujvo"), g("fu'ivla"), g("cmevla"), g("cmavo"), g("unknown")
        ),
        None => println!(
            "{{\"tokens\":{},\"gismu\":{},\"lujvo\":{},\"fu'ivla\":{},\"cmevla\":{},\"cmavo\":{},\"unknown\":{}}}",
            total, g("gismu"), g("lujvo"), g("fu'ivla"), g("cmevla"), g("cmavo"), g("unknown")
        ),
    }
    ExitCode::SUCCESS
}

/// --lines: 行単位バッチ解析(1行 = 1文)
fn run_lines(args: &Args, input: &str) -> ExitCode {
    let mut all_ok = true;
    for (i, line) in input.lines().enumerate() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        match lojban::parse(line) {
            Ok(pairs) => {
                if args.quiet {
                    // 無出力
                } else if args.json {
                    // 1行 = 1オブジェクト(JSONL)
                    println!("{}", tree::to_json(pairs));
                } else if args.sexpr {
                    println!("{}", tree::to_sexpr(pairs));
                } else {
                    println!("{}: ok", i + 1);
                }
            }
            Err(e) => {
                all_ok = false;
                let msg = lojban::friendly_error(&e);
                eprintln!("{}: {msg}", i + 1);
            }
        }
    }
    ExitCode::from(if all_ok { 0 } else { 1 })
}

/// 既定モード: 解析して指定形式で出力
fn run_parse(args: &Args, input: &str) -> ExitCode {
    match lojban::parse(input) {
        Ok(pairs) => {
            if args.quiet {
                // 成功時は無出力(終了コード 0)
            } else if args.json {
                println!("{}", tree::to_json(pairs));
            } else if args.dot {
                println!("{}", tree::to_dot(pairs));
            } else if args.html {
                println!("{}", tree::to_html(pairs));
            } else if args.sexpr {
                println!("{}", tree::to_sexpr(pairs));
            } else {
                println!("{}", tree::to_tree_string(pairs));
            }
            ExitCode::SUCCESS
        }
        Err(e) => {
            let msg = lojban::friendly_error(&e);
            eprintln!("{msg}");
            eprintln!("{e}");
            ExitCode::from(1)
        }
    }
}

/// 解析木中の sentence ノード数を数える(--stats 用)
fn count_sentences(pairs: &mut pest::iterators::Pairs<'_, Rule>) -> usize {
    let mut n = 0usize;
    for pair in pairs.by_ref() {
        if pair.as_rule() == Rule::sentence {
            n += 1;
        }
        n += count_sentences(&mut pair.into_inner());
    }
    n
}

/// 単語1語の語種を判定する(--classify / --stats 共用)
fn classify_word(word: &str) -> &'static str {
    [
        (Rule::jbocme, "cmevla"),
        (Rule::zifcme, "cmevla"),
        (Rule::gismu, "gismu"),
        (Rule::lujvo, "lujvo"),
        (Rule::fuhivla, "fu'ivla"),
        (Rule::cmavo, "cmavo"),
    ]
    .into_iter()
    .find(|(rule, _)| LojbanParser::parse(*rule, word).is_ok())
    .map(|(_, name)| name)
    .unwrap_or("unknown")
}

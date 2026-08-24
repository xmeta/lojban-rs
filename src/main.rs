//! ロジバン PEG パーサー CLI

use std::io::Read;
use std::process::ExitCode;

use clap::Parser as ClapParser;

use lojban::tree;

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

    if let Some(raw) = args.classify {
        use lojban::grammar::{LojbanParser, Rule};
        use pest::Parser;
        // 先頭のポーズ文字(. , ! ?)を除去
        let word = raw.trim_start_matches(['.', ',', '!', '?']);
        let class = [
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
        .unwrap_or("unknown");
        if args.json {
            println!("{{\"word\":\"{word}\",\"class\":\"{class}\"}}");
        } else {
            println!("{word}: {class}");
        }
        return ExitCode::SUCCESS;
    }

    if let Some(word) = args.split_lujvo {
        return match lojban::lujvo::decompose(&word) {
            Ok(parts) => {
                if args.json {
                    let items: Vec<String> = parts
                        .iter()
                        .map(|p| match p {
                            lojban::lujvo::Part::Rafsi { text, form } => format!(
                                "{{\"kind\":\"rafsi\",\"text\":\"{text}\",\"form\":\"{form:?}\"}}"
                            ),
                            lojban::lujvo::Part::Hyphen(c) => {
                                format!("{{\"kind\":\"hyphen\",\"char\":\"{c}\"}}")
                            }
                        })
                        .collect();
                    println!("{{\"word\":\"{word}\",\"parts\":[{}]}}", items.join(","));
                } else {
                    for p in parts {
                        match p {
                            lojban::lujvo::Part::Rafsi { text, form } => {
                                println!("{text} ({form:?})")
                            }
                            lojban::lujvo::Part::Hyphen(c) => println!("-{c}- [hyphen]"),
                        }
                    }
                }
                ExitCode::SUCCESS
            }
            Err(e) => {
                eprintln!("エラー: {e}");
                ExitCode::from(1)
            }
        };
    }

    if let Some(spec) = args.build_lujvo {
        let parts: Vec<&str> = spec
            .split(|c: char| c == ',' || c.is_ascii_whitespace())
            .filter(|s| !s.is_empty())
            .collect();
        return match lojban::lujvo::build(&parts) {
            Ok(built) => {
                if args.json {
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
        };
    }

    let input = match args.text.or_else(|| match &args.file {
        Some(path) => std::fs::read_to_string(path)
            .map_err(|e| eprintln!("エラー: {path} を読めませんでした: {e}"))
            .ok(),
        None => None,
    }) {
        Some(t) => t,
        None => {
            let mut buf = String::new();
            if std::io::stdin().read_to_string(&mut buf).is_err() {
                eprintln!("エラー: stdin の読み込みに失敗しました");
                return ExitCode::from(1);
            }
            if buf.trim().is_empty() {
                eprintln!("使い方: lojban [テキスト] [--sexpr]");
                return ExitCode::from(2);
            }
            buf
        }
    };

    if args.lines {
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
        return ExitCode::from(if all_ok { 0 } else { 1 });
    }

    match lojban::parse(&input) {
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

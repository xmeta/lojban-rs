//! ロジバン PEG パーサー CLI

use std::io::Read;
use std::process::ExitCode;

use clap::Parser as ClapParser;
use pest::error::ErrorVariant;

use lojban::grammar::Rule;
use lojban::tree;

/// 頻出する内部規則名を日本語の説明に変換する
fn rule_desc(rule: &Rule) -> String {
    match rule {
        Rule::BRIVLA_core | Rule::BRIVLA_clause => "内容語(brivla)".to_string(),
        Rule::CMEVLA_core | Rule::CMEVLA_clause => "固有名詞(cmevla)".to_string(),
        Rule::KOhA_core | Rule::KOhA_clause => "代名詞(sumti)".to_string(),
        Rule::LE_core | Rule::LE_clause => "冠詞(le/lo/la)".to_string(),
        Rule::selbri | Rule::tanru => "述語(selbri)".to_string(),
        Rule::sumti | Rule::sumti_core => "項(sumti)".to_string(),
        Rule::sentence => "文".to_string(),
        Rule::sep => "文接続(.i …)".to_string(),
        Rule::term => "項".to_string(),
        Rule::number | Rule::PA_core | Rule::PA_seq => "数詞".to_string(),
        Rule::PU_core
        | Rule::CAhA_core
        | Rule::ZAhO_core
        | Rule::ZI_core
        | Rule::VA_core
        | Rule::TAhE_core
        | Rule::ROI_core
        | Rule::FAhA_core => "時制詞".to_string(),
        Rule::UI_core | Rule::UINAI_joint => "感情標識".to_string(),
        Rule::BU_core => "bu(文字化)".to_string(),
        Rule::NU_core => "抽象(nu …)".to_string(),
        Rule::lu_quote | Rule::zo_quote | Rule::zoi_quote | Rule::lohu_quote => "引用".to_string(),
        other_rule => format!("{other_rule:?}"),
    }
}

/// pest のエラーに行位置の説明と日本語ヒントを添える
fn friendly_error(e: &pest::error::Error<Rule>) -> String {
    let (line, col) = match e.line_col {
        pest::error::LineColLocation::Pos((l, c)) => (l, c),
        pest::error::LineColLocation::Span((l, c), _) => (l, c),
    };
    let mut s = format!("解析エラー: {line} 行 {col} 列目付近");
    if let ErrorVariant::ParsingError { positives, .. } = &e.variant {
        let descs: Vec<String> = positives.iter().take(4).map(rule_desc).collect();
        if !descs.is_empty() {
            s.push_str(&format!(
                "\n  この位置では次の要素が可能: {}",
                descs.join(", ")
            ));
        }
    }
    s
}

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
    /// lujvo を生成する(rafsi を空白またはカンマ区切りで指定)
    #[arg(long)]
    build_lujvo: Option<String>,
    /// lujvo を rafsi 列に分解して表示
    #[arg(long)]
    split_lujvo: Option<String>,
}

fn main() -> ExitCode {
    let args = Args::parse();

    if let Some(word) = args.split_lujvo {
        return match lojban::lujvo::decompose(&word) {
            Ok(parts) => {
                for p in parts {
                    match p {
                        lojban::lujvo::Part::Rafsi { text, form } => {
                            println!("{text} ({form:?})")
                        }
                        lojban::lujvo::Part::Hyphen(c) => println!("-{c}- [hyphen]"),
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
                println!("{} (score {})", built.word, built.score());
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

    match lojban::parse(&input) {
        Ok(pairs) => {
            if args.json {
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
            let msg = friendly_error(&e);
            eprintln!("{msg}");
            eprintln!("{e}");
            ExitCode::from(1)
        }
    }
}

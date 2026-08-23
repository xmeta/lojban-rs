//! ロジバン PEG パーサー CLI

use std::io::Read;
use std::process::ExitCode;

use clap::Parser as ClapParser;

use lojban::tree;

#[derive(ClapParser, Debug)]
#[command(name = "lojban", version, about = "ロジバン PEG パーサー")]
struct Args {
    /// 解析するロジバンテキスト(未指定なら stdin)
    text: Option<String>,
    /// S 式形式で出力する
    #[arg(long)]
    sexpr: bool,
    /// JSON 形式で出力する
    #[arg(long)]
    json: bool,
    /// lujvo を生成する(rafsi を空白またはカンマ区切りで指定)
    #[arg(long)]
    build_lujvo: Option<String>,
}

fn main() -> ExitCode {
    let args = Args::parse();

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

    let input = match args.text {
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
            } else if args.sexpr {
                println!("{}", tree::to_sexpr(pairs));
            } else {
                println!("{}", tree::to_tree_string(pairs));
            }
            ExitCode::SUCCESS
        }
        Err(e) => {
            eprintln!("解析エラー: {e}");
            ExitCode::from(1)
        }
    }
}

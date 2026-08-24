//! CLI のエンドツーエンドテスト(実バイナリを起動して検証)。

use std::io::Write;
use std::process::{Command, Stdio};

fn run(args: &[&str], stdin: Option<&str>) -> (i32, String, String) {
    let mut child = Command::new(env!("CARGO_BIN_EXE_lojban"))
        .args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("バイナリ起動に失敗");
    if let Some(input) = stdin {
        child
            .stdin
            .as_mut()
            .unwrap()
            .write_all(input.as_bytes())
            .unwrap();
    }
    let out = child.wait_with_output().expect("実行に失敗");
    (
        out.status.code().unwrap_or(-1),
        String::from_utf8_lossy(&out.stdout).into_owned(),
        String::from_utf8_lossy(&out.stderr).into_owned(),
    )
}

#[test]
fn 基本解析_整形ツリー() {
    let (code, out, err) = run(&["mi klama do"], None);
    assert_eq!(code, 0, "{err}");
    assert!(out.contains("sentence"), "{out}");
}

#[test]
fn json出力はjsonl形式() {
    let (code, out, err) = run(&["--lines", "--json"], Some("mi klama do\ncoi\n"));
    assert_eq!(code, 0, "{err}");
    let lines: Vec<&str> = out.lines().collect();
    assert_eq!(lines.len(), 2, "{out}");
    for line in &lines {
        // 各行が JSON オブジェクトとして始まり version フィールドを持つ
        assert!(
            line.starts_with("{\"version\":1,\"rule\":\"text\""),
            "{line}"
        );
        assert!(line.ends_with('}'), "{line}");
    }
}

#[test]
fn quiet_成功時は無出力で終了コード0() {
    let (code, out, _err) = run(&["-q", "mi klama do"], None);
    assert_eq!(code, 0);
    assert!(out.is_empty(), "{out}");
}

#[test]
fn quiet_失敗時は終了コード1() {
    let (code, _out, err) = run(&["-q", "x y z"], None);
    assert_eq!(code, 1);
    assert!(err.contains("解析エラー"), "{err}");
}

#[test]
fn lines_モードは行番号を報告する() {
    let (code, out, err) = run(
        &["--lines"],
        Some("mi klama do\nle gerku cu cadzu\nx y z\n"),
    );
    assert_eq!(code, 1);
    assert!(out.contains("1: ok"), "{out}");
    assert!(out.contains("2: ok"), "{out}");
    assert!(err.contains("3:"), "{err}");
}

#[test]
fn file入力() {
    let dir = std::env::temp_dir().join("lojban_cli_test_file.txt");
    std::fs::write(&dir, "le mlatu cu cadzu").unwrap();
    let (code, out, err) = run(&["-f", dir.to_str().unwrap()], None);
    assert_eq!(code, 0, "{err}");
    assert!(out.contains("BRIVLA_core: \"mlatu\""), "{out}");
    let _ = std::fs::remove_file(&dir);
}

#[test]
fn build_lujvo_json() {
    let (code, out, err) = run(&["--build-lujvo", "zba sai", "--json"], None);
    assert_eq!(code, 0, "{err}");
    assert!(out.contains("\"word\":\"zbasai\""), "{out}");
    assert!(out.contains("\"score\":5847"), "{out}");
}

#[test]
fn split_lujvo_json() {
    let (code, out, err) = run(&["--split-lujvo", "sairzbata'u", "--json"], None);
    assert_eq!(code, 0, "{err}");
    assert!(out.contains("\"kind\":\"rafsi\""), "{out}");
    assert!(out.contains("\"kind\":\"hyphen\""), "{out}");
}

#[test]
fn dot出力() {
    let (code, out, err) = run(&["mi klama", "--dot"], None);
    assert_eq!(code, 0, "{err}");
    assert!(out.starts_with("digraph parse"), "{out}");
}

#[test]
fn html出力() {
    let (code, out, err) = run(&["mi klama", "--html"], None);
    assert_eq!(code, 0, "{err}");
    assert!(out.starts_with("<!DOCTYPE html>"), "{out}");
}

#[test]
fn classify_語種判定() {
    let cases = [
        ("klama", "gismu"),
        ("zbasai", "lujvo"),
        ("mi", "cmavo"),
        ("qqqzzz", "unknown"),
    ];
    for (word, class) in cases {
        // JSON 出力
        let (code, out, err) = run(&["--classify", word, "--json"], None);
        assert_eq!(code, 0, "{err}");
        assert!(out.contains(&format!("\"class\":\"{class}\"")), "{out}");
        // 既定は平文
        let (code, out, _err) = run(&["--classify", word], None);
        assert_eq!(code, 0);
        assert!(out.contains(class), "{out}");
    }
    // cmevla(先頭ポーズ付き)
    let (code, out, _err) = run(&["--classify", ".alis.", "--json"], None);
    assert_eq!(code, 0);
    assert!(out.contains("\"class\":\"cmevla\""), "{out}");
}

#[test]
fn stats_語種集計() {
    let dir = std::env::temp_dir().join("lojban_cli_test_stats.txt");
    std::fs::write(&dir, "mi klama do .i le gerku cu cadzu la alis.").unwrap();
    let (code, out, err) = run(&["--stats", "-f", dir.to_str().unwrap()], None);
    assert_eq!(code, 0, "{err}");
    assert!(out.contains("\"tokens\":10"), "{out}");
    assert!(out.contains("\"gismu\":3"), "{out}");
    assert!(out.contains("\"cmevla\":1"), "{out}");
    let _ = std::fs::remove_file(&dir);
}
